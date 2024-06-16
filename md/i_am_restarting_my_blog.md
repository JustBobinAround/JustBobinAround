+++
{
    "title": "I Am Restarting My Blog",
    "date": "2024-06-16T21:22:43.06Z"
}
+++
# I Am Restarting My Blog

So I restarted my blog. Its still a markdown blog, but I reduced the complexity
of it quite a bit. Instead of an SPA, it now just generates a series of html files
based on the markdown articles. I also added meta data for this markdown file,
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

Anyways... By having this metadata, now I can auto generate my list of articles
and have each one tagged with a date and title. This all gets templated in my
build system that I made for this site. Basically, I can run `make new` and
then it will create a new template and open the new markdown file in the user's
default editor. The internals for the build system can be found [here](https://github.com/JustBobinAround/JustBobinAround/tree/main/build_system).


