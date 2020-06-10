use super::add_skill_tree;

#[test]
fn adds_skill_tree() {
    let content = r#"# Chapter

```skill-tree
[[group]]
name = "test"
label = "test"
items = [ ]
```

Text
"#;

    let output = add_skill_tree(content, &mut 0).unwrap();
    println!("output:\n{}", output);
    assert!(output.contains(r#"<div id='skill-tree-0'></div>"#));
    assert!(output.contains(r#"<script>insertSkillTree("#));
}

#[test]
fn leaves_tables_untouched() {
    // Regression test.
    // Previously we forgot to enable the same markdwon extensions as mdbook itself.

    let content = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

    // Markdown roundtripping removes some insignificant whitespace
    let expected = r#"# Heading

|Head 1|Head 2|
|------|------|
|Row 1|Row 2|"#;

    assert_eq!(expected, add_skill_tree(content, &mut 0).unwrap());
}

#[test]
fn leaves_html_untouched() {
    // Regression test.
    // Don't remove important newlines for syntax nested inside HTML

    let content = r#"# Heading

<del>

*foo*

</del>
"#;

    // Markdown roundtripping removes some insignificant whitespace
    let expected = r#"# Heading

<del>

*foo*

</del>
"#;

    assert_eq!(expected, add_skill_tree(content, &mut 0).unwrap());
}
