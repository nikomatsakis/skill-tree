use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SkillTree {
    pub(crate) group: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) application: Option<String>,
    pub(crate) requires: Option<Vec<String>>,
    pub(crate) items: Vec<Item>,
    pub(crate) width: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) width: Option<f64>,
}

