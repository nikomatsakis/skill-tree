# skill-tree

A tool for rendering "skill trees", currently using graphviz.

## What is a skill tree?

A "skill tree" is a useful way to try and map out the "roadmap" for a
project. The term is borrowed from video games, but it was first
applied to project planning in this rather wonderful [blog post about
WebAssembly's post-MVP future][wasm] (at least, that was the first
time I'd seen it used that way).
[wasm]: https://hacks.mozilla.org/2018/10/webassemblys-post-mvp-future/

See an [example skill tree](https://nikomatsakis.github.io/skill-tree/)
in this project's website.

## How to use

### mdbook integration

The main way to use this project is to integrate it into an mdbook.
Use these steps to install it:

* `cargo install mdbook-skill-tree`
    * installs the `mdbook-skill-tree` executable
* in your mdbook's directory, `mdbook-skill-tree install`
    * updates your `book.toml` to contain the relevant javascript files
* in your mdbook, add a `skill-tree` code block, [as seen here](book/src/skill_tree.md).

## run manually

You can run `skill-tree` directly in which case it generates a `dot` file.
For example:

```bash
cargo run -- tree-data/example.toml example.dot
```

will transform the [`tree-data/example.toml`](tree-data/example.toml) 
file you can find in this repository.

## Next steps

I should, of course, create a skill-tree for this project-- but the
goal is to make this something that can be readily dropped into a
working group repository (like [wg-traits]) and used to track the
overall progress and plans of a project. The workflow isn't *quite*
drag and drop enough for that yet, but we're close!

[wg-traits]: https://github.com/rust-lang/wg-traits
