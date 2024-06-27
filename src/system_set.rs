use bevy::prelude::*;
use bevy_dolly::prelude::DollyUpdateSet;
use bevy_gltf_blueprints::GltfBlueprintsSet;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewSystemSet;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            (
                GltfBlueprintsSet::AfterSpawn,
                GameSystemSet::ColliderSpawn,
                GameSystemSet::Navigation,
                GameSystemSet::PlayerEmbodiment,
                GameSystemSet::GeneralMovement,
                GameSystemSet::PrepareAnimationState,
            )
                .chain(),
            (
                GameSystemSet::UpdateAnimationState,
                GameSystemSet::PlayAnimation,
                ExampleYarnSpinnerDialogueViewSystemSet,
                GameSystemSet::CameraUpdate,
                DollyUpdateSet,
                GameSystemSet::UpdateInteractionOpportunities,
            )
                .chain(),
        )
            .chain(),
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum GameSystemSet {
    /// - Goes through entities tagged with [`ColliderConstructor`](crate::physics::ColliderConstructor) in Blender
    /// - Inserts a proper XPBD collider
    /// - Inserts [`SensorLinks`](crate::physics::SensorLinks) on all [`RigidBody`](bevy_xpbd_3d::components::RigidBody)s
    /// - Links sensor colliders to their closest [`SensorLinks`](crate::physics::SensorLinks) up the hierarchy
    ColliderSpawn,
    /// Run path finding
    Navigation,
    /// Update interaction opportunities with the environment
    UpdateInteractionOpportunities,
    /// Handle player input
    PlayerEmbodiment,
    /// Handle movement for character controllers
    GeneralMovement,
    /// Prepare the exclusive animation state for the current frame
    PrepareAnimationState,
    /// Update the animation state according to this frame's events since [`GameSystemSet::PrepareAnimationState`]
    UpdateAnimationState,
    /// Play animations
    PlayAnimation,
    /// Update the camera transform
    CameraUpdate,
    /// Interacts with Yarn Spinner for dialog logic
    Dialog,
}
