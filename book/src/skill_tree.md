# Skill Tree

The "skill tree" package is a tool that you can use to plot out your overall trajectory. It is meant to be really easily integrated into `mdbook` but also usable stand-alone.

## Parts of a skill tree

* Groups
* Items

## Example

Here is an example of what a skill tree looks like. Check out [the source on github][src].

[src]: https://github.com/nikomatsakis/skill-tree/tree/master/example-trees/avd.toml

```
{{#include ../../example-trees/avd.toml}}
```

Here is how it looks in rendered form:

```skill-tree
{{#include ../../example-trees/avd.toml}}
```