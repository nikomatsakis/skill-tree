use crate::Fallible;
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
}

#[derive(Debug, Deserialize)]
pub(crate) struct Item {
    pub(crate) label: String,
    pub(crate) href: Option<String>,
    pub(crate) port: Option<String>,
    pub(crate) requires: Option<Vec<String>>,
}

impl SkillTree {
    pub(crate) fn validate(&self) -> Fallible<()> {
        // gather: valid requires entries

        for group in &self.group {
            group.validate()?;
        }
        Ok(())
    }
}

impl Group {
    pub(crate) fn validate(&self) -> Fallible<()> {
        // check: that `name` is a valid graphviz identifier

        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        for item in &self.items {
            item.validate()?;
        }

        Ok(())
    }
}

impl Item {
    pub(crate) fn validate(&self) -> Fallible<()> {
        // check: each of the things in requires has the form
        //        `identifier` or `identifier:port` and that all those
        //        identifiers map to groups

        // check: if you have a non-empty `requires`, must have a port

        Ok(())
    }
}
