+++
{
    "title": "I Am Restarting My Blog",
    "date": "2024-06-16T21:22:43.06Z"
}
+++
# I Am Restarting My Blog

So I restarted my blog. Its still a markdown blog, but I reduced the complexity
of it quite a bit. Instead of an SPA, it now just generates a series of html files
based on the markdown articles. I also added metadata for this markdown file,
so now my files have these headers:

```markdown
+++
{
    "title": "I Am Restarting My Blog",
    "date": "2024-06-16T21:22:43.06Z"
}
+++
# I Am Restarting My Blog
```

This feels very meta having my article in my article lol.

Anyways... By having this metadata, now I can auto-generate my list of articles
and have each one tagged with a date and title. This all gets templated in my
[build system](https://github.com/JustBobinAround/JustBobinAround/tree/main/build_system)
that I made for this site. Here is an overview of the Makefile
that I have set up:
```bash
serve: build
	cd ./build_system; cargo run -- serve 
    # This runs a http server that actively syncs the md files to html

build: 
	cd ./build_system; cargo run -- convert
    # This is for building if you were editing while the server wasn't running

new:
	cd ./build_system; cargo run -- new
    # This starts a prompt to create a new article
```

Makefiles, I know, they're scary. I always get a sinking feeling when I have to
build a large C project that uses things like CMake. For simple tasks like this
where I want to just create some simple commands for my project and have tab
completion support without converting my project to a full independent binary,
make files seem to be just fine. Depending on if I every feel up to it, I might
remove the make file and refine the actual rust program to be more suitable as
a terminal tool. That way anyone could just install it from cargo and spin up a
markdown blog.

### So Why Did I Change My Website Setup?

Well, I admit that I was actually working on a set of proc-macros that would build a
series of components for the Leptos framework based on a markdown file. I
still am working on them. They're basically done, I just haven't felt like
repackaging all the macros for cargo. Its a very cool idea, but as I started working
on it, I started to think: Why do I need an entire framework for a static page?
I love coding in Rust just like the next cultist masochist, but to what end? I just
want to document what I am doing in a fairly easy way. I really shouldn't try and
reinvent the wheel. I mean, I still will ship that project, I wrote the code...
But in the end, I don't think I will use it. I don't feel like shipping a giant
package of binaries and JS binding to the client just to render static text. So
instead, today I made this simple little build system. Now my page doesn't need
JavaScript or any other major complexity! 

### A Bit of a Tangent...

I really would like to make a full article about this at some point, but as a
short note... I think for most static pages, a good litmus test for a website
is if it works well in lynx or w3m. If you are not aware,
[lynx](https://lynx.invisible-island.net/) (and
[w3m](https://w3m.sourceforge.net/)) are programs that allow you to browse the
internet from you're terminal. I used to have slow internet, and an even slower
computer, so I would often resort to using lynx for reading articles. Using these
tools to view your page is also a good litmus test for how accessible the page
is for people that may have to use external software to help them read the page.

If you are feeling daring, try reading this page with lynx, then try reading
the first webpage recommended by google for cooking lasagna with lynx. Let me
know which is more readable lol.

### Back from the Tanget

Anyways, there isn't much left to talk about concerning my blog or its build
system at this point. I'll be sure to give an update if there are any major changes
to my workflow and/or software. Maybe I might make some voice transcription
software for this if I feel like it. For now though, that's all folks...
