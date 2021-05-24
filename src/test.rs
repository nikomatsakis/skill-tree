#![cfg(test)]

use std::path::PathBuf;

use crate::SkillTree;

const BLESS: bool = false;

fn run_test(file_name: &str) {
    let toml_file = PathBuf::from(format!("test-data/{}.toml", file_name));
    let skill_tree = SkillTree::load(&toml_file).unwrap();
    skill_tree.validate().unwrap();
    let mut actual_output_buf = Vec::new();
    skill_tree.write_graphviz(&mut actual_output_buf).unwrap();
    let actual_output = String::from_utf8(actual_output_buf).unwrap();

    let expected_file = PathBuf::from(format!("test-data/{}.gv", file_name));
    let expected_output = std::fs::read_to_string(&expected_file).unwrap_or_default();

    let expected_lines = expected_output.lines().chain(Some("EOF"));
    let actual_lines = actual_output.lines().chain(Some("EOF"));
    if let Some(first_diff) = expected_lines.zip(actual_lines).position(|(e, a)| e != a) {
        if BLESS || std::env::var("AVD_BLESS").is_ok() {
            std::fs::write(&expected_file, actual_output).unwrap();
            eprintln!("blessing {}", expected_file.display());
        } else {
            eprintln!(
                "{}",
                prettydiff::diff_lines(&expected_output, &actual_output)
            );
            panic!("expected output differs on line {}", first_diff + 1);
        }
    }
}

#[test]
fn avd_snippet() {
    run_test("avd_snippet");
}

#[test]
#[should_panic(expected = "the group `A` has a dependency on a group `B` that does not exist")]
fn invalid_requires() {
    run_test("invalid_requires");
}
