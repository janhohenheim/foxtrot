use bevy::prelude::*;
use bevy_dolly::prelude::DollyUpdateSet;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewSystemSet;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
                GltfBlueprintsSet::AfterSpawn,
                GameSystemSet::ColliderSpawn,
                GameSystemSet::UpdateInteractionOpportunities,
                GameSystemSet::Navigation,
                GameSystemSet::PlayerEmbodiment,
                GameSystemSet::GeneralMovement,
                GameSystemSet::PlayAnimation,
                GameSystemSet::Dialog,
                ExampleYarnSpinnerDialogueViewSystemSet,
                GameSystemSet::CameraUpdate,
                DollyUpdateSet,
        )
            .chain(),
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum GameSystemSet {
    /// Goes through entities tagged with `Collider` in Blender
    /// and inserts a proper XPBD collider
    ColliderSpawn,
    /// Run path finding
    Navigation,
    /// Update interaction opportunities with the environment
    UpdateInteractionOpportunities,
    /// Handle player input
    PlayerEmbodiment,
    /// Handle movement for character controllers
    GeneralMovement,
    /// Play animations
    PlayAnimation,
    /// Update the camera transform
    CameraUpdate,
    /// Interacts with Yarn Spinner for dialog logic
    Dialog,
}
