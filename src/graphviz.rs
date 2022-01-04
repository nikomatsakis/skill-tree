use crate::tree::{Graphviz, Group, ItemExt, SkillTree, Status};
use fehler::throws;
use std::io::Write;

impl SkillTree {
    /// Writes graphviz representing this skill-tree to the given output.
    #[throws(anyhow::Error)]
    pub fn write_graphviz(&self, output: &mut dyn Write) {
        write_graphviz(self, output)?
    }

    /// Generates a string containing graphviz content for this skill-tree.
    #[throws(anyhow::Error)]
    pub fn to_graphviz(&self) -> String {
        let mut output = Vec::new();
        write_graphviz(self, &mut output)?;
        String::from_utf8(output)?
    }
}

#[throws(anyhow::Error)]
fn write_graphviz(tree: &SkillTree, output: &mut dyn Write) {
    let rankdir = match &tree.graphviz {
        Some(Graphviz {
            rankdir: Some(rankdir),
            ..
        }) => &rankdir[..],
        _ => "LR",
    };
    writeln!(output, r#"digraph g {{"#)?;
    writeln!(
        output,
        r#"graph [ rankdir = "{rankdir}" ];"#,
        rankdir = rankdir
    )?;
    writeln!(output, r#"node [ fontsize="16", shape = "ellipse" ];"#)?;
    writeln!(output, r#"edge [ ];"#)?;

    if let Some(clusters) = &tree.cluster {
        for cluster in clusters {
            let cluster_name = format!("cluster_{}", cluster.name);
            writeln!(
                output,
                r#"subgraph {cluster_name} {{"#,
                cluster_name = cluster_name
            )?;
            writeln!(output, r#"    label="{}";"#, cluster.label)?;
            write_cluster(tree, output, Some(&cluster.name))?;
            writeln!(output, r#"}}"#)?;
        }
    }
    write_cluster(tree, output, None)?;

    for group in tree.groups() {
        if let Some(requires) = &group.requires {
            for requirement in requires {
                writeln!(output, r#""{}" -> "{}";"#, &group.name, requirement)?;
            }
        }
    }

    writeln!(output, r#"}}"#)?;
}

#[throws(anyhow::Error)]
fn write_cluster(tree: &SkillTree, output: &mut dyn Write, cluster: Option<&String>) {
    for group in tree.groups() {
        // If we are doing a cluster, the group must be in it;
        // otherwise, the group must not be in any cluster.
        match (&group.cluster, cluster) {
            (None, None) => {}
            (Some(c1), Some(c2)) if c1 == c2 => {}
            _ => continue,
        }
        writeln!(output, r#""{}" ["#, group.name)?;
        write_group_label(tree, group, output)?;
        writeln!(output, r#"  shape = "none""#)?;
        writeln!(output, r#"  margin = 0"#)?;
        writeln!(output, r#"]"#)?;
    }
}

const WATCH_EMOJI: &str = "⌚";
const HAMMER_WRENCH_EMOJI: &str = "🛠️";
const CHECKED_BOX_EMOJI: &str = "☑️";
const RAISED_HAND_EMOJI: &str = "🙋";

fn escape(s: &str) -> String {
    htmlescape::encode_minimal(s).replace('\n', "<br/>")
}

#[throws(anyhow::Error)]
fn write_group_label(tree: &SkillTree, group: &Group, output: &mut dyn Write) {
    writeln!(output, r#"  label = <<table>"#)?;

    let label = group.label.as_ref().unwrap_or(&group.name);
    let label = escape(label);
    let group_href = attribute_str("href", &group.href, "");
    let header_color = group
        .header_color
        .as_ref()
        .map(String::as_str)
        .unwrap_or("darkgoldenrod");
    let description_color = group
        .description_color
        .as_ref()
        .map(String::as_str)
        .unwrap_or("darkgoldenrod1");

    // We have one column for each thing specified by user, plus the label.
    let columns = tree.columns().len() + 1;

    writeln!(
        output,
        r#"    <tr><td bgcolor="{header_color}" colspan="{columns}"{group_href}>{label}</td></tr>"#,
        group_href = group_href,
        label = label,
        header_color = header_color,
        columns = columns,
    )?;

    for label in group.description.iter().flatten() {
        writeln!(
            output,
            r#"    <tr><td bgcolor="{description_color}" colspan="{columns}"{group_href}>{label}</td></tr>"#,
            group_href = group_href,
            label = label,
            description_color = description_color,
            columns = columns,
        )?;
    }

    for item in &group.items {
        let item_status = Status::Unassigned; // XXX
        let (_emoji, _fontcolor, mut start_tag, mut end_tag) = match item_status {
            Status::Blocked => (
                WATCH_EMOJI,
                None,
                "<i><font color=\"lightgrey\">",
                "</font></i>",
            ),
            Status::Unassigned => (RAISED_HAND_EMOJI, Some("red"), "", ""),
            Status::Assigned => (HAMMER_WRENCH_EMOJI, None, "", ""),
            Status::Complete => (CHECKED_BOX_EMOJI, None, "<s>", "</s>"),
        };

        let bgcolor = attribute_str("bgcolor", &Some("cornsilk"), "");
        let href = attribute_str("href", &item.href(), "");
        if item.href().is_some() && start_tag == "" {
            start_tag = "<u>";
            end_tag = "</u>";
        }
        write!(output, "    <tr>")?;

        for column in tree.columns() {
            let item_value = item.column_value(tree, column);
            let emoji = tree.emoji(column, item_value);
            write!(
                output,
                "<td{bgcolor}>{emoji}</td>",
                bgcolor = bgcolor,
                emoji = emoji
            )?;
        }

        write!(
            output,
            "<td{bgcolor}{href}>{start_tag}{label}{end_tag}</td>",
            bgcolor = bgcolor,
            href = href,
            label = item.label(),
            start_tag = start_tag,
            end_tag = end_tag,
        )?;

        writeln!(output, "</tr>")?;
    }

    writeln!(output, r#"  </table>>"#)?;
}

fn attribute_str(label: &str, text: &Option<impl AsRef<str>>, suffix: &str) -> String {
    match text {
        None => format!(""),
        Some(t) => format!(" {}=\"{}{}\"", label, t.as_ref(), suffix),
    }
}
