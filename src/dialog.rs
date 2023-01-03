use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::utils::{HashMap, HashSet};
use bevy_egui::EguiPlugin;
use std::fs;
use std::path::Path;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<ActiveConditions>()
            .add_event::<DialogEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_dialog))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_current_dialog)
                    .with_system(show_dialog),
            );
    }
}

fn init_dialog(mut dialog_event_writer: EventWriter<DialogEvent>) {
    dialog_event_writer.send(DialogEvent(DialogId::new("sample")))
}

fn set_current_dialog(
    mut commands: Commands,
    active_conditions: Res<ActiveConditions>,
    mut dialog_events: EventReader<DialogEvent>,
) {
    for DialogEvent(id) in dialog_events.iter() {
        let dialog = load_dialog(id);
        let current_page = dialog
            .initial_page
            .iter()
            .filter(|page| {
                page.positive_requirements.is_subset(&active_conditions.0)
                    && page.negative_requirements.is_disjoint(&active_conditions.0)
            })
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "No valid active page for dialog {:?}. Current conditions: {:?}",
                    id, active_conditions
                )
            })
            .id
            .clone();
        commands.insert_resource(CurrentDialog {
            dialog,
            current_page,
        });
    }
}

fn show_dialog(current_dialog: Option<Res<CurrentDialog>>) {
    let current_dialog = match current_dialog {
        Some(current_dialog) => current_dialog,
        None => return,
    };
    let _current_page = current_dialog
        .dialog
        .pages
        .get(&current_dialog.current_page)
        .unwrap();
}

#[derive(Debug)]
pub struct DialogEvent(DialogId);

#[derive(Resource, Default, Debug)]
pub struct ActiveConditions(HashSet<ConditionId>);

#[derive(Resource, Debug)]
pub struct CurrentDialog {
    dialog: Dialog,
    current_page: PageId,
}

fn load_dialog(id: &DialogId) -> Dialog {
    let filename = format!("{}.json", id.0);
    let path = Path::new("assets").join("dialogs").join(filename);
    let json = fs::read_to_string(path.clone())
        .unwrap_or_else(|e| panic!("Failed to open dialog file at {:?}: {}", path, e));
    serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("Failed to parse dialog file at {:?}: {}", path, e))
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page {
    /// If `None`, this is a dummy page for other pages to converge to while still showing their own text.
    /// This means the text of the last page will be displayed.
    pub text: Option<String>,
    pub next_page: NextPage,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NextPage {
    /// There is only one automatic option for the next page
    Continue(PageId),
    /// The user can choose between different answers that determine the next page
    Choice(HashMap<ConditionId, DialogChoice>),
    /// Exit dialog after this page
    Exit,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DialogChoice {
    /// The player's answer
    pub text: String,
    pub next_page: PageId,
    #[serde(default)]
    pub positive_requirements: HashSet<ConditionId>,
    #[serde(default)]
    pub negative_requirements: HashSet<ConditionId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ConditionId(pub String);
impl ConditionId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DialogId(pub String);
impl DialogId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PageId(pub String);
impl PageId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}
