use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::utils::{HashMap, HashSet};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct DialogEvent(pub DialogId);

#[derive(Resource, Default, Debug)]
pub struct ActiveConditions(pub HashSet<ConditionId>);

#[derive(Resource, Debug)]
pub struct CurrentDialog {
    pub dialog: Dialog,
    pub current_page: PageId,
    pub last_choice: Option<ConditionId>,
}
impl CurrentDialog {
    pub fn fetch_page(&self, page_id: &PageId) -> Page {
        self.dialog
            .pages
            .get(&page_id)
            .unwrap_or_else(|| panic!("Failed to fetch page with id {}", page_id.0))
            .clone()
    }
    pub fn fetch_current_page(&self) -> Page {
        self.fetch_page(&self.current_page)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dialog {
    pub initial_page: Vec<InitialPage>,
    pub pages: HashMap<PageId, Page>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InitialPage {
    pub id: PageId,
    #[serde(default)]
    pub positive_requirements: HashSet<ConditionId>,
    #[serde(default)]
    pub negative_requirements: HashSet<ConditionId>,
}

impl InitialPage {
    pub fn is_available(&self, active_conditions: &ActiveConditions) -> bool {
        self.positive_requirements.is_subset(&active_conditions.0)
            && self.negative_requirements.is_disjoint(&active_conditions.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DialogChoice {
    /// The player's answer
    pub text: String,
    pub next_page_id: PageId,
    #[serde(default)]
    pub positive_requirements: HashSet<ConditionId>,
    #[serde(default)]
    pub negative_requirements: HashSet<ConditionId>,
}

impl DialogChoice {
    pub fn is_available(&self, active_conditions: &ActiveConditions) -> bool {
        self.positive_requirements.is_subset(&active_conditions.0)
            && self.negative_requirements.is_disjoint(&active_conditions.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConditionId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DialogId(pub String);
impl DialogId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PageId(pub String);
