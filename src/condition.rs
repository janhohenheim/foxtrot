use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

pub struct ConditionPlugin;

impl Plugin for ConditionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveConditions>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct ActiveConditions(pub HashSet<ConditionId>);

#[derive(Debug, Clone, Eq, PartialEq, Default, Reflect, Hash, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ConditionId(pub String);
