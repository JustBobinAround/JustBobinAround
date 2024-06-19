+++
{
    "title": "The Best K NeoVim Remap",
    "date": "2024-06-17T16:53:37.06Z"
}
+++
# The Best K NeoVim Remap
If you aren't a NeoVim user, I am sorry for you're loss. If you are a NeoVim
user, keep reading, this is for you:

If you read the title carefully, I am talking about remapping "K", not "k". So
for those that are unaware, "K" usually will try and run a man.lua file for the
current word that your cursor is over. This is great, and I want to keep this
functionality. But for certain languages, like rust or JavaScript, most of the
time I am reading documentation from these three websites:

- [docs.rs](https://docs.rs/)
- [developer.mozilla.org](https://developer.mozilla.org/en-US/)
- [docs.servicenow.com](https://docs.servicenow.com) - for my job

So my normal usages will usually involve the task of copying something from my code,
googling the copied text, and maybe adding a few refining keywords or site filters.
It may sound like that is a short task, but the time it takes to do this does
add up after a while. So after thinking about it for a bit, I remembered that
services like google and duckduckgo have the "I'm feeling lucky" operation.
This operation will instantly redirect you to the most likely web result. Once I
remembered this, I thought: Why not modify "K" to do this instead?

So that's what I did for Rust files:

```lua
vim.api.nvim_create_autocmd({ "BufEnter", "BufWinEnter" }, {
  pattern = { "*.rs"},
  callback = function()
	vim.keymap.set("n", "K", [[:!firefox https\:\/\/duckduckgo.com\/\?t=h\_\&q=\!+<C-r><C-w>+site\:docs\.rs<CR>]])
  end,
})
```

as well as for JavaScript files:

```lua
vim.api.nvim_create_autocmd({ "BufEnter", "BufWinEnter" }, {
  pattern = { "*.js"},
  callback = function()
	vim.keymap.set("n", "K", [[:!firefox https\:\/\/duckduckgo.com\/\?t=h\_\&q=\!+<C-r><C-w>+site\:developer\.mozilla\.org+OR+site\:docs\.servicenow\.com<CR>]])
  end,
})
```

So what's going on here? Well, ignoring the terrible wrapping because I suck at css, NeoVim
is taking the current word (just like normal "K") and then building a "I'm feeling lucky" link
with duckduckgo - I am using duckduckgo because google seems to make more of a fuss
when I try and do this - I have specified different link criteria for each file type i.e.:

```
site:docs.rs                                            // for Rust

site:developer.mozilla.org+OR+site:docs.servicenow.com  // for JS
```

So that way, the chances of me landing at the correct page are higher. Once the
link is built, I can then launch the link in Firefox:
```lua
[[:!firefox <LINK HERE>]]
```

I am thinking about building this out to be a full plugin system at some point.
The issue is that I am also trying to build my own text editor, so I am trying
to keep my distance from relying on vim as my long term dev tool. Its a difficult
line to draw.

### A Caveat 

Something that I have noticed with vim is that when using this:

```lua
vim.api.nvim_create_autocmd({ "BufEnter", "BufWinEnter" }, {
```
along with telescope, things seem to not fully work when switching buffers via fuzzy
finder. The autocmd seems to only run post saving after switching buffers. If
anyone knows a way around this please let me know. 

### After Thoughts

I will probably keep writing about NeoVim even though I eventually plan to switch
to my own editor because to be honest, the editor is pretty much just going to 
be a NeoVim clone but just built into a terminal emulator for easier copy-paste
support. So if you enjoyed this article, stay tuned as there will be more NeoVim
articles and:

j̶o̶i̶n̶ ̶m̶y̶ ̶n̶e̶w̶ ̶l̶e̶t̶t̶e̶r̶

If you have a blog, don't do that, its annoying. Nobody make money on blogs anymore
anyways. Either include an rss feed for the nerds or just hope your audience comes
back. Don't spam emails.

Anyways, that's all folks.


