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
	python -m http.server

build: 
	cd ./build_system; cargo run -- convert

new:
	cd ./build_system; cargo run -- new

push: build
	git add .
	git commit -m "running build"
	git push

```
Makefiles, I know, they're scary. I always get a sinking feeling when I have to
build a large C project that uses things like CMake. For simple tasks like this
where I want to just create some simple commands for my project and have tab
completion support, make files seem to be just fine. It feels a little bit strange
having a git push command in my make file, but this seems to be fine since I'm
really just syncing changes from my markdown files to my html files. Depending
on if I every feel up to it, I might remove the make file and refine the actual
rust program to be more suitable as a terminal tool. That way anyone could just
install it from cargo and spin up a markdown blog.

### So Why Did I Change My Website Setup?

Well, I admit that I was actually working on a set of proc-macros that would build a
series of components for the Leptos framework based on your markdown files. I
still am working on them and they're basically done, I just haven't felt like
repackaging all the macros for cargo. Its a very cool idea, but as I started working
on it, I started to think: Why do I need an entire framework for a static page.
I love coding in Rust just like the next cultist masochist, but to what end? I just
want to document what I am doing in a fairly easy way. I really shouldn't try and
reinvent the wheel. I mean, I still will ship that project, but I don't think I will
use it. I don't feel like shipping a giant package binaries and JS binding to the
client just to render static text. So instead, today I made this simple little build
system. Now my page doesn't need JavaScript or any other major complexity! 

### A Bit of a Tangent...

I really would like to make a full article about this at some point, but as a
short note... I really think for most static pages, a good litmus test for the
site is if it works well in lynx or w3m. If you are not aware,
[lynx](https://lynx.invisible-island.net/) (and
[w3m](https://w3m.sourceforge.net/)) are programs that allow you to browse the
internet from you're terminal. I used to have slow internet, and an even slower
computer, so I would often resort to using lynx for reading article.

