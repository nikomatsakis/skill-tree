# Skill Tree

The "skill tree" package is a tool that you can use to plot out your overall
trajectory. It is meant to be really easily integrated into `mdbook` but also
usable stand-alone. 

## Example

Here is an example of what skill tree's look like:

```skill-tree
[[group]]
name = "align-rustc-predicate"
label = "Align rustc predicates with chalk predicates"
items = [
  { label = "isolate Binder into a Forall goal" },
  { label = "introduce Implication" },
  { label = "introduce Forall goals with types" },
]

[[group]]
name = "recursive-solver"
label = "Experiment with a recursive chalk solver"
items = [
  { label = "write-up the idea that Niko had" },
  { label = "build prototype and evaluate" },
]
```