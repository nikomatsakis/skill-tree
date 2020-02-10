use std::fs::File;
use std::path::PathBuf;

mod graphviz;
mod tree;

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

#[paw::main]
fn main(args: paw::Args) -> Fallible<()> {
    for arg in args.skip(1) {
        let path = PathBuf::from(arg);
        println!("{:?}", path);
        let text = std::fs::read_to_string(&path)?;
        let skill_tree: tree::SkillTree = toml::from_str(&text)?;

        let output_path = path.with_extension("dot");
        println!("{:#?}", skill_tree);
        println!("{:#?}", output_path);

        skill_tree.validate()?;

        let mut file = File::create(output_path)?;
        graphviz::write_graphviz(&skill_tree, &mut file)?;
    }

    Ok(())
}
