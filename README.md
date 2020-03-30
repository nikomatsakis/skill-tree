# skill-tree

A tool for rendering "skill trees", currently using graphviz.

## What is a skill tree?

A "skill tree" is a useful way to try and map out the "roadmap" for a
project. The term is borrowed from video games, but it was first
applied to project planning in this rather wonderful [blog post about
WebAssembly's post-MVP future][wasm] (at least, that was the first
time I'd seen it used that way).

[wasm]: https://hacks.mozilla.org/2018/10/webassemblys-post-mvp-future/

## Example



## How does this project work?

You create a [TOML file](tree-data/example.toml) that defines the
"groups" (major work areas) along with the items in each group. These
items can be linked to github issues or other urls, and you can create
dependencies between groups and items. Here is an [example of such a
file](tree-data/example.toml).

You then run the tool like so:

```bash
> cargo run -- tree-data/example.toml output
```

This will generate a directory `output` that contains various
javascript and dot files. If you view `output/skill-tree.html` you
will get a view of the complete skill tree. You can also load
`output/skill-tree.js` (along with the `viz.js` scripts) and execute
the skill tree from some context of your own.

In particular, `output/skill-tree.dot` is your graphviz file that you
can use to render on your own if you prefer.

## Next steps

I should, of course, create a skill-tree for this project-- but the
goal is to make this something that can be readily dropped into a
working group repository (like [wg-traits]) and used to track the
overall progress and plans of a project. The workflow isn't *quite*
drag and drop enough for that yet, but we're close!

[wg-traits]: https://github.com/rust-lang/wg-traits
