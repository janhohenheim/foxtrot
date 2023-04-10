use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

pub(crate) fn condition_plugin(app: &mut App) {
    app.init_resource::<ActiveConditions>()
        .add_event::<ConditionAddEvent>()
        .add_system(add_conditions.in_set(OnUpdate(GameState::Playing)));
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct ActiveConditions(pub(crate) HashSet<ConditionId>);
impl ActiveConditions {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, Reflect, Hash, Serialize, Deserialize, FromReflect,
)]
#[reflect(Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub(crate) struct ConditionId(pub(crate) String);

impl From<String> for ConditionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ConditionId> for String {
    fn from(value: ConditionId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Reflect, Hash, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct ConditionAddEvent(pub(crate) ConditionId);

fn add_conditions(
    mut conditions: ResMut<ActiveConditions>,
    mut incoming_conditions: EventReader<ConditionAddEvent>,
) {
    for incoming_condition in incoming_conditions.iter() {
        conditions.0.insert(incoming_condition.0.clone());
    }
}
