# skill-tree

A tool for rendering "skill trees", currently using graphviz.

## What is a skill tree?

A "skill tree" is a useful way to try and map out the "roadmap" for a
project. The term is borrowed from video games, but it was first
applied to project planning in this rather wonderful [blog post about
WebAssembly's post-MVP future][wasm] (at least, that was the first
time I'd seen it used that way).

[wasm]: https://hacks.mozilla.org/2018/10/webassemblys-post-mvp-future/

## How does this project work?

You create a [TOML file](tree-data/traits.toml) that defines the
"groups" (major work areas) along with the items in each group. These
items can be linked to github issues or other urls, and you can create
dependencies between groups and items. Here is an [example of such a
file](tree-data/traits.toml).

You then run the tool like so:

```bash
> cargo run -- tree-data/traits.toml
```

This will generate a `tree-data/traits.dot` file. This file can be rendered to an SVG
like so (you have to have installed [graphviz])

[graphviz]: https://www.graphviz.org/

```bash
> dot -Tsvg tree-data/traits.dot  > traits.svg
```

which will generate an SVG file ([example](tree-data/traits.svg)).


