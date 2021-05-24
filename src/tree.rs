use fehler::throws;
use serde_derive::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Deserialize)]
pub struct SkillTree {
    pub group: Vec<Group>,
    pub cluster: Option<Vec<Cluster>>,
    pub graphviz: Option<Graphviz>,
    pub doc: Option<Doc>,
}

#[derive(Debug, Deserialize)]
pub struct Graphviz {
    pub rankdir: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Doc {
    pub columns: Vec<String>,
    pub defaults: Option<HashMap<String, String>>,
    pub emoji: Option<HashMap<String, EmojiMap>>,
}

pub type EmojiMap = HashMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub label: String,
    pub color: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
    pub cluster: Option<String>,
    pub label: Option<String>,
    pub requires: Option<Vec<String>>,
    pub description: Option<Vec<String>>,
    pub items: Vec<Item>,
    pub width: Option<f64>,
    pub status: Option<Status>,
    pub href: Option<String>,
    pub header_color: Option<String>,
    pub description_color: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GroupIndex(pub usize);

pub type Item = HashMap<String, String>;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ItemIndex(pub usize);

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Status {
    /// Can't work on it now
    Blocked,

    /// Would like to work on it, but need someone
    Unassigned,

    /// People are actively working on it
    Assigned,

    /// This is done!
    Complete,
}

impl SkillTree {
    #[throws(anyhow::Error)]
    pub fn load(path: &Path) -> SkillTree {
        let skill_tree_text = std::fs::read_to_string(path)?;
        Self::parse(&skill_tree_text)?
    }

    #[throws(anyhow::Error)]
    pub fn parse(text: &str) -> SkillTree {
        toml::from_str(text)?
    }

    #[throws(anyhow::Error)]
    pub fn validate(&self) {
        // gather: valid requires entries

        for group in &self.group {
            group.validate(self)?;
        }
    }

    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.group.iter()
    }

    pub fn group_named(&self, name: &str) -> Option<&Group> {
        self.group.iter().find(|g| g.name == name)
    }

    /// Returns the expected column titles for each item (excluding the label).
    pub fn columns(&self) -> &[String] {
        if let Some(doc) = &self.doc {
            &doc.columns
        } else {
            &[]
        }
    }

    /// Translates an "input" into an emoji, returning "input" if not found.
    pub fn emoji<'me>(&'me self, column: &str, input: &'me str) -> &'me str {
        if let Some(doc) = &self.doc {
            if let Some(emoji_maps) = &doc.emoji {
                if let Some(emoji_map) = emoji_maps.get(column) {
                    if let Some(output) = emoji_map.get(input) {
                        return output;
                    }
                }
            }
        }
        input
    }
}

impl Group {
    #[throws(anyhow::Error)]
    pub fn validate(&self, tree: &SkillTree) {
        // check: that `name` is a valid graphviz identifier

        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        for group_name in self.requires.iter().flatten() {
            if tree.group_named(group_name).is_none() {
                anyhow::bail!(
                    "the group `{}` has a dependency on a group `{}` that does not exist",
                    self.name,
                    group_name,
                )
            }
        }

        for item in &self.items {
            item.validate()?;
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }
}

pub trait ItemExt {
    fn href(&self) -> Option<&String>;
    fn label(&self) -> &String;
    fn column_value<'me>(&'me self, tree: &'me SkillTree, c: &str) -> &'me str;

    #[allow(redundant_semicolons)] // bug in "throws"
    #[throws(anyhow::Error)]
    fn validate(&self);
}

impl ItemExt for Item {
    fn href(&self) -> Option<&String> {
        self.get("href")
    }

    fn label(&self) -> &String {
        self.get("label").unwrap()
    }

    fn column_value<'me>(&'me self, tree: &'me SkillTree, c: &str) -> &'me str {
        if let Some(v) = self.get(c) {
            return v;
        }

        if let Some(doc) = &tree.doc {
            if let Some(defaults) = &doc.defaults {
                if let Some(default_value) = defaults.get(c) {
                    return default_value;
                }
            }
        }

        ""
    }

    #[throws(anyhow::Error)]
    fn validate(&self) {
        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        // check: only contains known keys
    }
}
