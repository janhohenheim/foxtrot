use crate::system_set::GameSystemSet;
use crate::util::{single, single_mut};
use crate::{
    level_instantiation::on_spawn::Player,
    player_control::{
        actions::{ActionsFrozen, PlayerAction},
        camera::{IngameCamera, IngameCameraKind},
    },
    util::is_frozen,
    world_interaction::dialog::{CurrentDialogTarget, YarnNode},
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};
use bevy_xpbd_3d::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use std::iter;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionOpportunity>()
        .init_resource::<InteractionOpportunity>()
        .add_systems(
            Update,
            (update_interaction_opportunities, display_interaction_prompt)
                .chain()
                .in_set(GameSystemSet::UpdateInteractionOpportunities)
                .run_if(not(is_frozen)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
struct InteractionOpportunity(Option<Entity>);

fn update_interaction_opportunities(
    player_query: Query<(&GlobalTransform, &CollidingEntities), With<Player>>,
    parents: Query<&Parent>,
    target_query: Query<
        (Entity, &GlobalTransform),
        (With<YarnNode>, Without<Player>, Without<IngameCamera>),
    >,
    camera_query: Query<(&IngameCamera, &GlobalTransform), Without<Player>>,
    mut interaction_opportunity: ResMut<InteractionOpportunity>,
) {
    interaction_opportunity.0 = None;
    let (player_transform, collisions) = single!(player_query);
    let player_translation = player_transform.translation();
    let (camera, camera_transform) = single!(camera_query);

    for &sensor in collisions.0.iter() {
        // A dialog collider is valid for any of its ancestors
        let mut ancestors = iter::once(sensor).chain(parents.iter_ancestors(sensor));

        // Check if what we are colliding with is a dialog target
        let Some((target, target_transform)) =
            ancestors.find_map(|entity| target_query.get(entity).ok())
        else {
            continue;
        };

        // Check if we are facing the right way
        let is_facing_target = is_facing_target(
            player_translation,
            target_transform.translation(),
            camera_transform.compute_transform(),
            camera,
        );
        if is_facing_target {
            interaction_opportunity.0.replace(target);
            break;
        }
    }
}

fn is_facing_target(
    player: Vec3,
    target: Vec3,
    camera_transform: Transform,
    camera: &IngameCamera,
) -> bool {
    if camera.kind == IngameCameraKind::FixedAngle {
        return true;
    }
    let camera_to_player = camera_transform.forward();
    let player_to_target = target - player;
    let angle = camera_to_player.angle_between(player_to_target);
    angle < TAU / 8.
}

fn display_interaction_prompt(
    interaction_opportunity: Res<InteractionOpportunity>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut egui_contexts: EguiContexts,
    actions: Query<&ActionState<PlayerAction>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    dialog_target_query: Query<(Entity, &YarnNode)>,
    mut freeze: ResMut<ActionsFrozen>,
    mut current_dialog_target: ResMut<CurrentDialogTarget>,
) {
    let Some(opportunity) = interaction_opportunity.0 else {
        return;
    };
    let window = single!(primary_windows);
    let mut dialogue_runner = single_mut!(dialogue_runner);

    let (entity, dialog_target) = dialog_target_query.get(opportunity).unwrap();
    egui::Window::new("Interaction")
        .collapsible(false)
        .title_bar(false)
        .auto_sized()
        .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.label("E: Talk");
        });
    for actions in actions.iter() {
        if actions.just_pressed(&PlayerAction::Interact) {
            dialogue_runner.start_node(&dialog_target.0);
            current_dialog_target.0.replace(entity);
            freeze.freeze();
        }
    }
}
