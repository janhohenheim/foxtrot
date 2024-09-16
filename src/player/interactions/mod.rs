use crate::system_set::VariableGameSystem;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

mod interact;
mod prompt;
mod update_available;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(
        PlayerInteractionParameters,
        AvailablePlayerInteraction,
        PlayerInteraction,
    )>();
    app.add_plugins((prompt::plugin, interact::plugin, update_available::plugin));
    app.configure_sets(
        Update,
        (
            OpportunitySystem::UpdateAvailableOpportunities,
            OpportunitySystem::ShowPrompt,
            OpportunitySystem::Interact,
        )
            .chain()
            .in_set(VariableGameSystem::Opportunities),
    );
}

/// Systems related to opportunities.
///
/// Opportunities are actions that the player can perform, such as interacting with a door.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum OpportunitySystem {
    /// Updates the list of available opportunities.
    UpdateAvailableOpportunities,
    /// Shows or hides the prompt for the best available opportunity.
    ShowPrompt,
    /// Handles the player interacting with the best available opportunity.
    Interact,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Default, Deref, DerefMut, Reflect)]
#[reflect(Component, PartialEq, Default)]
pub struct AvailablePlayerInteraction(pub Option<Entity>);

/// The general idea is as follows:
/// This component sits on a collider for an interactable object, e.g. a door or a character.
/// Every update, we send a raycast from the camera's forward direction to see if it hits a
/// [`PotentialOpportunity`] collider.
/// If so, we have an interaction opportunity.
#[derive(Debug, Component, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PlayerInteractionParameters {
    /// The prompt to display when the opportunity is available.
    pub prompt: String,
    /// The maximum distance from the camera at which the opportunity can be interacted with.
    pub max_distance: f32,
}

impl PlayerInteractionParameters {
    pub fn default(player_interaction: &PlayerInteraction) -> Self {
        match player_interaction {
            PlayerInteraction::Dialog(..) => Self {
                prompt: "Talk".to_string(),
                max_distance: 2.5,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(PartialEq, Component)]
pub enum PlayerInteraction {
    /// A dialog opportunity with a Yarn Spinner dialogue node.
    Dialog(String),
}

impl Component for PlayerInteraction {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            if world.get::<PlayerInteractionParameters>(entity).is_some() {
                return;
            }
            let interaction = world.get::<PlayerInteraction>(entity).unwrap();
            let parameters = PlayerInteractionParameters::default(interaction);
            world.commands().entity(entity).insert(parameters);
        });
    }
}
