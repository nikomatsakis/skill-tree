use crate::tree::{Group, SkillTree};
use fehler::throws;
use std::io::Write;

#[throws(anyhow::Error)]
pub(crate) fn write_graphviz(tree: &SkillTree, output: &mut dyn Write) {
    writeln!(output, r#"digraph g {{"#)?;
    writeln!(output, r#"graph [ rankdir = "LR" ];"#)?;
    writeln!(output, r#"node [ fontsize="16", shape = "ellipse" ];"#)?;
    writeln!(output, r#"edge [ ];"#)?;

    for group in &tree.group {
        writeln!(output, r#""{}" ["#, group.name)?;
        write_group_label(group, output)?;
        writeln!(output, r#"  shape = "none""#)?;
        writeln!(output, r#"  margin = 0"#)?;
        writeln!(output, r#"]"#)?;
    }

    for group in &tree.group {
        if let Some(requires) = &group.requires {
            for requirement in requires {
                writeln!(
                    output,
                    r#"{} -> {};"#,
                    port_name(requirement),
                    port_name(&group.name),
                )?;
            }
        }

        for item in &group.items {
            if let Some(requires) = &item.requires {
                for requirement in requires {
                    writeln!(
                        output,
                        r#"{} -> "{}":{};"#,
                        port_name(requirement),
                        group.name,
                        item.port.as_ref().expect("missing port"),
                    )?;
                }
            }
        }
    }

    writeln!(output, r#"}}"#)?;
}

#[throws(anyhow::Error)]
fn write_group_label(group: &Group, output: &mut dyn Write) {
    writeln!(output, r#"  label = <<table>"#)?;

    let label = group.label.as_ref().unwrap_or(&group.name);

    writeln!(
        output,
        r#"    <tr><td bgcolor="gray" port="all">{}</td></tr>"#,
        label,
    )?;

    for item in &group.items {
        let href = attribute_str("href", &item.href);
        let port = attribute_str("port", &item.port);
        writeln!(
            output,
            r#"    <tr><td{}{}>{}</td></tr>"#,
            href, port, item.label
        )?;
    }

    writeln!(output, r#"  </table>>"#)?;
}

fn port_name(requires: &str) -> String {
    if let Some(index) = requires.find(":") {
        let name = &requires[..index];
        let port = &requires[index + 1..];
        format!(r#""{}":{}"#, name, port)
    } else {
        format!(r#""{}":all"#, requires)
    }
}

fn attribute_str(label: &str, text: &Option<String>) -> String {
    match text {
        None => format!(""),
        Some(t) => format!(" {}=\"{}\"", label, t),
    }
}
