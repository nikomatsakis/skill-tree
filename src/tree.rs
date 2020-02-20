use fehler::throws;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SkillTree {
    pub(crate) group: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) label: Option<String>,
    pub(crate) application: Option<String>,
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

    pub(crate) fn groups(&self) -> impl Iterator<Item = &Group> {
        self.group.iter()
    }

    pub(crate) fn groups_enumerated(&self) -> impl Iterator<Item = (&Group, GroupIndex)> {
        self.group.iter().zip((0..).map(GroupIndex))
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

    pub(crate) fn items_enumerated(&self) -> impl Iterator<Item = (&Item, ItemIndex)> {
        self.items.iter().zip((0..).map(ItemIndex))
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
