use anyhow::Context;
use fehler::throws;
use rust_embed::RustEmbed;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod graphviz;
mod tree;

#[derive(StructOpt, Debug)]
#[structopt(name = "skill-tree")]
struct Opts {
    #[structopt(name = "skill_tree", parse(from_os_str))]
    skill_tree: PathBuf,

    #[structopt(name = "output_dir", parse(from_os_str))]
    output_dir: PathBuf,
}

// Embed the contents of the viz-js folder at build time
#[derive(RustEmbed)]
#[folder = "viz-js/"]
struct VizJs;

#[throws(anyhow::Error)]
fn main() {
    let opts: Opts = Opts::from_args();

    // Load the skill tree
    let skill_tree = load_skill_tree(&opts.skill_tree)
        .with_context(|| format!("loading skill tree from `{}`", opts.skill_tree.display()))?;

    // Validate it for errors.
    skill_tree.validate()?;

    // Create the output directory
    std::fs::create_dir_all(&opts.output_dir)
        .with_context(|| format!("creating output directory `{}`", opts.output_dir.display()))?;

    // Write out the dot file
    write_dot_file(&skill_tree, &opts)?;

    // Copy into it the content from viz-js folder
    for file in VizJs::iter() {
        let file: &str = file.as_ref();
        let file_text = VizJs::get(file).unwrap();
        let file_text: &[u8] = file_text.as_ref();
        let output_path = opts.output_dir.join(file);
        write_static_file(&output_path, file_text)
            .with_context(|| format!("creating static file `{}`", output_path.display()))?;
    }
}

#[throws(anyhow::Error)]
fn load_skill_tree(path: &Path) -> tree::SkillTree {
    let skill_tree_text = std::fs::read_to_string(path)?;
    toml::from_str(&skill_tree_text)?
}

#[throws(anyhow::Error)]
fn write_dot_file(skill_tree: &tree::SkillTree, opts: &Opts) {
    let dot_path = &opts.output_dir.join("skill-tree.dot");
    let mut dot_file =
        File::create(dot_path).with_context(|| format!("creating `{}`", dot_path.display()))?;
    graphviz::write_graphviz(&skill_tree, &mut dot_file)
        .with_context(|| format!("writing to `{}`", dot_path.display()))?;
}

#[throws(anyhow::Error)]
fn write_static_file(output_path: &Path, file_text: &[u8]) {
    let mut file = File::create(output_path)?;
    file.write_all(&file_text)?;
}
