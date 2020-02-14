use rust_embed::RustEmbed;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod graphviz;
mod tree;

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

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

fn main() -> Fallible<()> {
    let opts: Opts = Opts::from_args();

    // Load the skill tree
    let skill_tree_text = std::fs::read_to_string(&opts.skill_tree)?;
    let skill_tree: tree::SkillTree = toml::from_str(&skill_tree_text)?;

    // Validate it for errors.
    skill_tree.validate()?;

    // Create the output directory
    println!("creating directory `{}`", opts.output_dir.display());
    std::fs::create_dir_all(&opts.output_dir)?;

    // Write out the dot file
    println!("writing dot file");
    write_dot_file(&skill_tree, &opts)?;

    // Copy into it the content from viz-js folder
    for file in VizJs::iter() {
        let file: &str = file.as_ref();
        let file_text = VizJs::get(file).unwrap();
        let file_text: &[u8] = file_text.as_ref();
        let output_path = opts.output_dir.join(file);
        let mut file = File::create(dbg!(output_path))?;
        file.write_all(&file_text)?;
    }

    Ok(())
}

fn write_dot_file(skill_tree: &tree::SkillTree, opts: &Opts) -> Fallible<()> {
    let dot_path = opts.output_dir.join("skill-tree.dot");
    let mut dot_file = File::create(dbg!(dot_path))?;
    graphviz::write_graphviz(&skill_tree, &mut dot_file)?;
    Ok(())
}
