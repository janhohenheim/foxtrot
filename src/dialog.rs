use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_egui::EguiPlugin;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).init_resource::<Dialogs>();
    }
}

#[derive(Resource, Default)]
pub struct Dialogs(HashMap<DialogId, Dialog>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dialog {
    pub initial_page: Vec<InitialPage>,
    pub pages: HashMap<PageId, Page>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitialPage {
    pub id: PageId,
    pub positive_requirements: HashSet<ConditionId>,
    pub negative_requirements: HashSet<ConditionId>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    /// If `None`, this is a dummy page for other pages to converge to while still showing their own text.
    /// This means the text of the last page will be displayed.
    pub text: Option<String>,
    pub next_page: NextPage,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NextPage {
    /// There is only one automatic option for the next page
    Continue(PageId),
    /// The user can choose between different answers that determine the next page
    Choice(HashMap<ConditionId, DialogChoice>),
    /// Exit dialog after this page
    Exit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DialogChoice {
    /// The player's answer
    pub text: String,
    pub next_page: PageId,
    pub positive_requirements: HashSet<ConditionId>,
    pub negative_requirements: HashSet<ConditionId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConditionId(pub String);
impl ConditionId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DialogId(pub String);
impl DialogId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PageId(pub String);
impl PageId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}
