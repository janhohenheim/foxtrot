use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

pub fn dialog_running(query: Query<&DialogueRunner>) -> bool {
    query.iter().any(|dialogue_runner| dialogue_runner.is_running())
}
