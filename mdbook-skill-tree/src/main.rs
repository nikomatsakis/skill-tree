use anyhow::Context;
use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use fehler::throws;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};
use toml_edit::{value, Array, Document, Item, Table, Value};

mod preprocessor;
use preprocessor::SkillTreePreprocessor;

struct JsFile {
    name: &'static str,
    bytes: &'static [u8],
}

const JS_FILES: &[JsFile] = &[
    JsFile {
        name: "full.render.js",
        bytes: include_bytes!("../js/full.render.js"),
    },
    JsFile {
        name: "skill-tree.js",
        bytes: include_bytes!("../js/skill-tree.js"),
    },
    JsFile {
        name: "viz.js",
        bytes: include_bytes!("../js/viz.js"),
    },
];

pub fn make_app() -> App<'static, 'static> {
    App::new("mdbook-skill-tree")
        .version(crate_version!())
        .about("mdbook preprocessor to add skill-tree support")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .arg(
                    Arg::with_name("dir")
                    .default_value(".")
                    .help("Root directory for the book,\nshould contain the configuration file (`book.toml`)")
                    )
                .about("]Install the required assset files and include it in the config"),
        )
}

#[throws(anyhow::Error)]
fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("install") {
        handle_install(sub_args)?;
    } else {
        if let Err(e) = handle_preprocessing() {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-mermaid preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = SkillTreePreprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = SkillTreePreprocessor.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

#[throws(anyhow::Error)]
fn handle_install(sub_args: &ArgMatches) {
    let dir = sub_args.value_of("dir").expect("Required argument");
    let proj_dir = PathBuf::from(dir);
    let config = proj_dir.join("book.toml");

    if !config.exists() {
        log::error!("Configuration file '{}' missing", config.display());
        process::exit(1);
    }

    log::info!("Reading configuration file {}", config.display());
    let toml = fs::read_to_string(&config).expect("can't read configuration file");
    let mut doc = toml
        .parse::<Document>()
        .expect("configuration is not valid TOML");

    let has_pre = has_preprocessor(&mut doc);
    if !has_pre {
        log::info!("Adding preprocessor configuration");
        add_preprocessor(&mut doc);
    }

    let added_additional_files = add_additional_files(&mut doc);
    if !has_pre || added_additional_files {
        log::info!("Saving changed configuration to {}", config.display());
        let toml = doc.to_string_in_original_order();
        let mut file = File::create(config).expect("can't open configuration file for writing.");
        file.write_all(toml.as_bytes())
            .expect("can't write configuration");
    }

    let mut printed = false;

    // Copy into it the content from viz-js folder
    for file in JS_FILES {
        let output_path = proj_dir.join(file.name);
        if output_path.exists() {
            log::debug!(
                "'{}' already exists (Path: {}). Skipping.",
                file.name,
                output_path.display()
            );
            continue;
        }
        if !printed {
            printed = true;
            log::info!(
                "Writing additional files to project directory at {}",
                proj_dir.display()
            );
        }
        log::debug!(
            "Writing content for '{}' into {}",
            file.name,
            output_path.display()
        );
        write_static_file(&output_path, file.bytes)
            .with_context(|| format!("creating static file `{}`", output_path.display()))?;
    }

    log::info!("Files & configuration for mdbook-skill-tree are installed. You can start using it in your book.");
}

#[throws(anyhow::Error)]
fn write_static_file(output_path: &Path, file_text: &[u8]) {
    let mut file = File::create(output_path)?;
    file.write_all(&file_text)?;
}

fn add_additional_files(doc: &mut Document) -> bool {
    let mut changed = false;
    let mut printed = true;

    for file in JS_FILES {
        let additional_js = additional(doc, "js");
        if has_file(&additional_js, file.name) {
            log::debug!("'{}' already in 'additional-js'. Skipping", file.name)
        } else {
            if !printed {
                printed = true;
                log::info!("Adding additional files to configuration");
            }
            log::debug!("Adding '{}' to 'additional-js'", file.name);
            insert_additional(doc, "js", file.name);
            changed = true;
        }
    }

    changed
}

fn additional<'a>(doc: &'a mut Document, additional_type: &str) -> Option<&'a mut Array> {
    let doc = doc.as_table_mut();

    let item = doc.entry("output");
    let item = item
        .or_insert(empty_implicit_table())
        .as_table_mut()?
        .entry("html");
    let item = item
        .or_insert(empty_implicit_table())
        .as_table_mut()?
        .entry(&format!("additional-{}", additional_type));
    item.as_array_mut()
}

fn has_preprocessor(doc: &mut Document) -> bool {
    matches!(doc["preprocessor"]["skill-tree"], Item::Table(_))
}

fn empty_implicit_table() -> Item {
    let mut empty_table = Table::default();
    empty_table.set_implicit(true);
    Item::Table(empty_table)
}

fn add_preprocessor(doc: &mut Document) {
    let doc = doc.as_table_mut();

    let item = doc.entry("preprocessor").or_insert(empty_implicit_table());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("skill-tree")
        .or_insert(empty_implicit_table());
    item["command"] = value("mdbook-skill-tree");
}

fn has_file(elem: &Option<&mut Array>, file: &str) -> bool {
    match elem {
        Some(elem) => elem.iter().any(|elem| match elem.as_str() {
            None => true,
            Some(s) => s.ends_with(file),
        }),
        None => false,
    }
}

fn insert_additional(doc: &mut Document, additional_type: &str, file: &str) {
    let doc = doc.as_table_mut();

    let empty_table = Item::Table(Table::default());
    let empty_array = Item::Value(Value::Array(Array::default()));
    let item = doc.entry("output").or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("html")
        .or_insert(empty_table.clone());
    let array = item
        .as_table_mut()
        .unwrap()
        .entry(&format!("additional-{}", additional_type))
        .or_insert(empty_array);
    array
        .as_value_mut()
        .unwrap()
        .as_array_mut()
        .unwrap()
        .push(file);
}
