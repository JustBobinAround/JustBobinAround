use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use std::sync::mpsc::channel;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::net::TcpListener;use pulldown_cmark::{html, Parser as PCParser};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;
use clap::{Parser, Subcommand};
use chrono::Utc;

#[derive(Parser)]
#[command(name = "Markdown Converter")]
#[command(about = "Converts Markdown files to HTML with metadata embedding")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New,
    Convert,
    CreateBlog,
    Serve
}

macro_rules! article_template {
    ($title: expr, $content: expr) => {
        format!(include_str!("./template.html"), $title, $content)        
    };
    ($title: literal, $content: expr) => {
        format!(include_str!("./template.html"), $title, $content)        
    };
}

macro_rules! title_template {
    ($date: expr, $title: expr, $url: expr) => {
        {format!("- [**{}: {}**](/JustBobinAround/{})\n", $date, $title, $url)}
    };
}

#[derive(Debug, Deserialize)]
struct MetaData {
    title: String,
    date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let input_dir = "../md";
    let output_dir = "../html";

    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateBlog => {
            unimplemented!("haven't made this feature yet, sorry");
        }
        Commands::New=> {
            let mut input = String::new();
            println!("Enter Article Name:");
            if io::stdin().read_line(&mut input).is_ok() {
                create_markdown_template(&input)?;
            }
        }
        Commands::Convert => {
            convert_markdown_files(input_dir, output_dir)?;
        }
        Commands::Serve => {
            run_server(&input_dir, &output_dir).await?;
        }
    }

    Ok(())
}

async fn run_server(input_dir: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let (tx, rx) = channel();
    let config = Config::default()
        .with_poll_interval(Duration::from_secs(2))
        .with_compare_contents(true);

    // Automatically select the best implementation for your platform.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, config)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("../md"), RecursiveMode::Recursive)?;

    let listener = TcpListener::bind(addr).await?;
    let input_dir = input_dir.to_owned();
    let output_dir = output_dir.to_owned();
    tokio::task::spawn(async move {
        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("Files changed, rebuilding...");
                    convert_markdown_files(&input_dir,&output_dir);
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_req))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }

    Ok(())
}
fn create_markdown_template(article_name: &str) -> io::Result<()> {
    let formatted_date = Utc::now()
        .format("%Y-%m-%dT%H:%M:%S.%mZ")
        .to_string();

    let metadata = format!(
        "+++\n{{\n    \"title\": \"{}\",\n    \"date\": \"{}\"\n}}\n+++\n# {}",
        article_name.trim(),
        formatted_date,
        article_name.trim()
    );

    let file_name = format!("../md/{}.md", article_name.replace(" ", "_").to_lowercase().trim());
    let mut file = File::create(&file_name)?;
    write!(file, "{}", metadata)?;


    let editor_var = env!("EDITOR");
    if let Ok(mut join_handler) = Command::new(editor_var)
        .arg(file_name)
        .spawn() {
        let _ = join_handler.wait();
    }


    println!("Markdown template created successfully.");
    Ok(())
}


fn convert_markdown_files(input_dir: &str, output_dir: &str) -> io::Result<()>{
    fs::create_dir_all(output_dir)?;

    let mut articles: String = String::new();

    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            process_markdown_file(&mut articles, path, output_dir)?;
        }
    }
    let parser = PCParser::new(&articles);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let final_output = article_template!("Article List", html_output);

    let output_file_path = Path::new("../html/article_list").with_extension("html");
    let mut output_file = File::create(output_file_path)?;
    write!(output_file, "{}", final_output)?;

    let parser = PCParser::new(include_str!("../../README.md"));
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let final_output = article_template!("JustBobinAround's Blog", html_output);

    let output_file_path = Path::new("../index").with_extension("html");
    let mut output_file = File::create(output_file_path)?;
    write!(output_file, "{}", final_output)?;

    Ok(())
}

fn process_markdown_file(articles: &mut String, path: &Path, output_dir: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut metadata = String::new();
    let mut markdown_content = String::new();
    let mut in_metadata = false;
    let mut metadata_count = 0;
    

    for line in reader.lines() {
        let line = line?;
        if line.trim() == "+++" && metadata_count < 2 {
            in_metadata = !in_metadata;
            metadata_count += 1;
            continue;
        }

        if in_metadata {
            metadata.push_str(&line);
            metadata.push('\n');
        } else {
            markdown_content.push_str(&line);
            markdown_content.push('\n');
        }
    }

    let metadata: MetaData = serde_json::from_str(&metadata).expect("Metadata missing from article");

    let parser = PCParser::new(&markdown_content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let final_output = article_template!(metadata.title, html_output);

    let output_file_path = Path::new(output_dir).join(path.file_stem().unwrap()).with_extension("html");
    if let Some(output_file_path) = output_file_path.to_str() {
        if output_file_path.find("../").is_some_and(|i|i==0) {
            let output_file_path = output_file_path.replacen("../", "", 1);
            articles.push_str(&title_template!(metadata.date, metadata.title, output_file_path));
        } else {
            articles.push_str(&title_template!(metadata.date, metadata.title, output_file_path));
        }
    }
    let mut output_file = File::create(output_file_path)?;
    write!(output_file, "{}", final_output)?;

    Ok(())
}

async fn handle_req(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = format!("..{}", req.uri().path());
    let path = path.replacen("../JustBobinAround/", "../",1);
    let path = if path == "../" { "../index.html".to_string() } else { path };
    println!("{:?}", path);
    if Path::new(&path).exists() && Path::new(&path).is_file() {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => {content},
            Err(_) => {String::new()}
        };
        let mut response = Response::new(Full::new(Bytes::from(content)));
        response.headers_mut().insert("Refresh", HeaderValue::from_static("5"));
        Ok(response)
    } else {
        Ok(Response::new(Full::new(Bytes::from("404"))))
    }
}


