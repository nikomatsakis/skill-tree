use crate::tree::{Goal, Group, SkillTree, Status};
use fehler::throws;
use std::io::Write;

#[throws(anyhow::Error)]
pub(crate) fn write_graphviz(tree: &SkillTree, output: &mut dyn Write) {
    writeln!(output, r#"digraph g {{"#)?;
    writeln!(output, r#"graph [ rankdir = "LR" ];"#)?;
    writeln!(output, r#"node [ fontsize="16", shape = "ellipse" ];"#)?;
    writeln!(output, r#"edge [ ];"#)?;

    for group in tree.groups() {
        writeln!(output, r#""{}" ["#, group.name)?;
        write_group_label(group, output)?;
        writeln!(output, r#"  shape = "none""#)?;
        writeln!(output, r#"  margin = 0"#)?;
        writeln!(output, r#"]"#)?;
    }

    for goal in tree.goals() {
        writeln!(output, r#""{}" ["#, goal.name)?;
        write_goal_label(goal, output)?;
        writeln!(output, r#"  shape = "note""#)?;
        writeln!(output, r#"  margin = 0"#)?;
        writeln!(output, r#"  style = "filled""#)?;
        writeln!(output, r#"  fillcolor = "darkgoldenrod""#)?;
        writeln!(output, r#"]"#)?;
    }

    for group in tree.groups() {
        if let Some(requires) = &group.requires {
            for requirement in requires {
                writeln!(
                    output,
                    r#"{} -> {};"#,
                    tree.port_name(requirement, "out"),
                    tree.port_name(&group.name, "in"),
                )?;
            }
        }

        for item in group.items() {
            if let Some(requires) = &item.requires {
                for requirement in requires {
                    writeln!(
                        output,
                        r#"{} -> "{}":{};"#,
                        tree.port_name(requirement, "out"),
                        group.name,
                        item.port.as_ref().expect("missing port"),
                    )?;
                }
            }
        }
    }

    for goal in tree.goals() {
        if let Some(requires) = &goal.requires {
            for requirement in requires {
                writeln!(
                    output,
                    r#"{} -> {};"#,
                    tree.port_name(requirement, "out"),
                    tree.port_name(&goal.name, "in"),
                )?;
            }
        }
    }

    writeln!(output, r#"}}"#)?;
}

const WATCH_EMOJI: &str = "⌚";
const HAMMER_WRENCH_EMOJI: &str = "🛠️";
const CHECKED_BOX_EMOJI: &str = "☑️";
const RAISED_HAND_EMOJI: &str = "🙋";

fn escape(s: &str) -> String {
    htmlescape::encode_minimal(s).replace('\n', "<br/>")
}

#[throws(anyhow::Error)]
fn write_goal_label(goal: &Goal, output: &mut dyn Write) {
    let label = goal.label.as_ref().unwrap_or(&goal.name);
    let label = escape(label);
    writeln!(output, r#"  label = "{label}""#, label = label)?;
}

#[throws(anyhow::Error)]
fn write_group_label(group: &Group, output: &mut dyn Write) {
    writeln!(output, r#"  label = <<table>"#)?;

    let label = group.label.as_ref().unwrap_or(&group.name);
    let label = escape(label);
    let group_href = attribute_str("href", &group.href, "");

    writeln!(
        output,
        r#"    <tr><td bgcolor="darkgoldenrod" port="all" colspan="2"{group_href}>{label}</td></tr>"#,
        group_href = group_href,
        label = label,
    )?;

    for item in &group.items {
        let item_status = item.status.or(group.status).unwrap_or(Status::Unassigned);
        let (emoji, fontcolor, mut start_tag, mut end_tag) = match item_status {
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

        let fontcolor = attribute_str("fontcolor", &fontcolor, "");
        let bgcolor = attribute_str("bgcolor", &Some("cornsilk"), "");
        let href = attribute_str("href", &item.href, "");
        if item.href.is_some() && start_tag == "" {
            start_tag = "<u>";
            end_tag = "</u>";
        }
        let port_in = attribute_str("port", &item.port, "_in");
        let port_out = attribute_str("port", &item.port, "_out");
        writeln!(
            output,
            "    \
             <tr>\
             <td{bgcolor}{port_in}>{emoji}</td>\
             <td{fontcolor}{bgcolor}{href}{port_out}>\
             {start_tag}{label}{end_tag}\
             </td>\
             </tr>",
            fontcolor = fontcolor,
            bgcolor = bgcolor,
            emoji = emoji,
            href = href,
            port_in = port_in,
            port_out = port_out,
            label = item.label,
            start_tag = start_tag,
            end_tag = end_tag,
        )?;
    }

    writeln!(output, r#"  </table>>"#)?;
}

fn attribute_str(label: &str, text: &Option<impl AsRef<str>>, suffix: &str) -> String {
    match text {
        None => format!(""),
        Some(t) => format!(" {}=\"{}{}\"", label, t.as_ref(), suffix),
    }
}

impl SkillTree {
    fn port_name(&self, requires: &str, mode: &str) -> String {
        if let Some(index) = requires.find(":") {
            let name = &requires[..index];
            let port = &requires[index + 1..];
            format!(r#""{}":{}_{}"#, name, port, mode)
        } else if self.is_goal(requires) {
            // Goals don't have ports, so we don't need a `:all`
            format!(r#""{}""#, requires)
        } else {
            format!(r#""{}":all"#, requires)
        }
    }
}
