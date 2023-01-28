use crate::world_interaction::condition::{ActiveConditions, ConditionId};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct DialogEvent {
    pub dialog: DialogId,
    pub page: Option<PageId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Serialize, Deserialize, Default)]
pub struct CurrentDialog {
    pub id: DialogId,
    pub dialog: Dialog,
    pub current_page: PageId,
    pub last_choice: Option<ConditionId>,
}
impl CurrentDialog {
    pub fn fetch_page(&self, page_id: &PageId) -> Page {
        self.dialog
            .pages
            .get(page_id)
            .unwrap_or_else(|| panic!("Failed to fetch page with id {}", page_id.0))
            .clone()
    }
    pub fn fetch_current_page(&self) -> Page {
        self.fetch_page(&self.current_page)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Dialog {
    pub initial_page: Vec<InitialPage>,
    pub pages: HashMap<PageId, Page>,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub struct InitialPage {
    pub id: PageId,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub positive_requirements: HashSet<ConditionId>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub negative_requirements: HashSet<ConditionId>,
}

impl InitialPage {
    pub fn is_available(&self, active_conditions: &ActiveConditions) -> bool {
        self.positive_requirements.is_subset(&active_conditions.0)
            && self.negative_requirements.is_disjoint(&active_conditions.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Page {
    pub text: String,
    pub next_page: NextPage,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NextPage {
    /// There is only one automatic option for the next page
    Continue(PageId),
    /// The user can choose between different answers that determine the next page
    Choice(IndexMap<ConditionId, DialogChoice>),
    /// Use `next_page` of the specified `Page`
    SameAs(PageId),
    /// Exit dialog after this page
    Exit,
}
impl Default for NextPage {
    fn default() -> Self {
        Self::Exit
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct DialogChoice {
    /// The player's answer
    pub text: String,
    pub next_page_id: PageId,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub positive_requirements: HashSet<ConditionId>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub negative_requirements: HashSet<ConditionId>,
}

impl DialogChoice {
    pub fn is_available(&self, active_conditions: &ActiveConditions) -> bool {
        self.positive_requirements.is_subset(&active_conditions.0)
            && self.negative_requirements.is_disjoint(&active_conditions.0)
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, Component, Reflect, Hash, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct DialogId(pub String);
impl DialogId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for DialogId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DialogId> for String {
    fn from(value: DialogId) -> Self {
        value.0
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, Reflect, FromReflect, Hash, Serialize, Deserialize,
)]
#[reflect(Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct PageId(pub String);

impl From<String> for PageId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<PageId> for String {
    fn from(value: PageId) -> Self {
        value.0
    }
}
