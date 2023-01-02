use bevy::prelude::*;
use bevy::utils::HashMap;
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
    pages: HashMap<PageId, Page>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Page {
    text: String,
    next_page: NextPage,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NextPage {
    Forced(PageId),
    Chosen(HashMap<ChoiceId, Choice>),
    Exit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Choice {
    pub text: String,
    pub next_page: NextPage,
    pub positive_requirements: Vec<ChoiceId>,
    pub negative_requirements: Vec<ChoiceId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChoiceId(pub String);
impl ChoiceId {
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
