use fehler::throws;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SkillTree {
    pub(crate) group: Vec<Group>,
    pub(crate) goal: Option<Vec<Goal>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Goal {
    pub(crate) name: String,
    pub(crate) label: Option<String>,
    pub(crate) requires: Option<Vec<String>>,
    pub(crate) href: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) label: Option<String>,
    pub(crate) requires: Option<Vec<String>>,
    pub(crate) items: Vec<Item>,
    pub(crate) width: Option<f64>,
    pub(crate) status: Option<Status>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GroupIndex(pub usize);

#[derive(Debug, Deserialize)]
pub(crate) struct Item {
    pub(crate) label: String,
    pub(crate) href: Option<String>,
    pub(crate) port: Option<String>,
    pub(crate) requires: Option<Vec<String>>,
    pub(crate) status: Option<Status>,
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
    pub(crate) fn validate(&self) {
        // gather: valid requires entries

        for group in &self.group {
            group.validate()?;
        }
    }

    pub(crate) fn is_goal(&self, name: &str) -> bool {
        self.goals().any(|goal| goal.name == name)
    }

    pub(crate) fn goals(&self) -> impl Iterator<Item = &Goal> {
        self.goal.iter().flat_map(|v| v.iter())
    }

    pub(crate) fn groups(&self) -> impl Iterator<Item = &Group> {
        self.group.iter()
    }
}

impl Group {
    #[throws(anyhow::Error)]
    pub(crate) fn validate(&self) {
        // check: that `name` is a valid graphviz identifier

        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        for item in &self.items {
            item.validate()?;
        }
    }

    pub(crate) fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }
}

impl Item {
    #[throws(anyhow::Error)]
    pub(crate) fn validate(&self) {
        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        // check: if you have a non-empty `requires`, must have a port
    }
}
