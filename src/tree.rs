use fehler::throws;
use serde_derive::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SkillTree {
    pub group: Vec<Group>,
    pub goal: Option<Vec<Goal>>,
}

#[derive(Debug, Deserialize)]
pub struct Goal {
    pub name: String,
    pub label: Option<String>,
    pub requires: Option<Vec<String>>,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
    pub label: Option<String>,
    pub requires: Option<Vec<String>>,
    pub items: Vec<Item>,
    pub width: Option<f64>,
    pub status: Option<Status>,
    pub href: Option<String>,
    pub header_color: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GroupIndex(pub usize);

#[derive(Debug, Deserialize)]
pub struct Item {
    pub label: String,
    pub href: Option<String>,
    pub port: Option<String>,
    pub requires: Option<Vec<String>>,
    pub status: Option<Status>,
}

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
        toml::from_str(&skill_tree_text)?
    }

    #[throws(anyhow::Error)]
    pub fn validate(&self) {
        // gather: valid requires entries

        for group in &self.group {
            group.validate()?;
        }
    }

    pub fn is_goal(&self, name: &str) -> bool {
        self.goals().any(|goal| goal.name == name)
    }

    pub fn goals(&self) -> impl Iterator<Item = &Goal> {
        self.goal.iter().flat_map(|v| v.iter())
    }

    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.group.iter()
    }
}

impl Group {
    #[throws(anyhow::Error)]
    pub fn validate(&self) {
        // check: that `name` is a valid graphviz identifier

        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        for item in &self.items {
            item.validate()?;
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }
}

impl Item {
    #[throws(anyhow::Error)]
    pub fn validate(&self) {
        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        // check: if you have a non-empty `requires`, must have a port
    }
}
