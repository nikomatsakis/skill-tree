use anyhow::Context;
use fehler::throws;
use skill_tree::SkillTree;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "skill-tree")]
struct Opts {
    #[structopt(name = "skill_tree", parse(from_os_str))]
    skill_tree: PathBuf,

    #[structopt(name = "output_dir", parse(from_os_str))]
    output_dir: PathBuf,
}

#[throws(anyhow::Error)]
fn main() {
    let opts: Opts = Opts::from_args();

    // Load the skill tree
    let skill_tree = SkillTree::load(&opts.skill_tree)
        .with_context(|| format!("loading skill tree from `{}`", opts.skill_tree.display()))?;

    // Validate it for errors.
    skill_tree.validate()?;

    // Create the output directory
    std::fs::create_dir_all(&opts.output_dir)
        .with_context(|| format!("creating output directory `{}`", opts.output_dir.display()))?;

    // Write out the dot file
    write_dot_file(&skill_tree, &opts)?;
}

#[throws(anyhow::Error)]
fn write_dot_file(skill_tree: &SkillTree, opts: &Opts) {
    let dot_path = &opts.output_dir.join("skill-tree.dot");
    let mut dot_file =
        File::create(dot_path).with_context(|| format!("creating `{}`", dot_path.display()))?;
    skill_tree
        .write_graphviz(&mut dot_file)
        .with_context(|| format!("writing to `{}`", dot_path.display()))?;
}

#[throws(anyhow::Error)]
fn write_static_file(output_path: &Path, file_text: &[u8]) {
    let mut file = File::create(output_path)?;
    file.write_all(&file_text)?;
}
