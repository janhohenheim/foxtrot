use crate::player_control::actions::{ActionsFrozen, PlayerAction};
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::player_control::player_embodiment::Player;
use crate::util::criteria::is_frozen;
use crate::world_interaction::dialog::DialogTarget;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_mod_sysfail::*;
use bevy_xpbd_3d::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub(crate) fn interactions_ui_plugin(app: &mut App) {
    app.register_type::<InteractionOpportunity>()
        .init_resource::<InteractionOpportunity>()
        .add_systems(
            Update,
            (update_interaction_opportunities)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            display_interaction_prompt.run_if(
                not(is_frozen)
                    .and_then(in_state(GameState::Playing))
                    .and_then(any_with_component::<DialogueRunner>()),
            ),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct InteractionOpportunity(pub(crate) Option<Entity>);

#[sysfail(log(level = "error"))]
fn update_interaction_opportunities(
    mut collisions: EventReader<Collision>,
    player_query: Query<Entity, With<Player>>,
    player_transform_query: Query<&Transform, With<Player>>,
    non_player_query: Query<&Transform, (Without<Player>, Without<IngameCamera>)>,
    camera_query: Query<(&IngameCamera, &Transform), Without<Player>>,
    mut interaction_opportunity: ResMut<InteractionOpportunity>,
) -> Result<()> {
    interaction_opportunity.0 = None;
    for event in collisions.read() {
        // Check if we are colliding
        let (entity_a, entity_b, ongoing) = unpack_event(event);
        if !ongoing {
            continue;
        }

        let Some(target_entity) = get_target_entity(&player_query, entity_a, entity_b) else {
            continue;
        };

        // Check if we are facing the right way
        let target_transform = non_player_query
            .get(target_entity)
            .context("Failed to get transform of interaction target")?;
        for player_transform in player_transform_query.iter() {
            for (camera, camera_transform) in camera_query.iter() {
                let is_facing_target = is_facing_target(
                    *player_transform,
                    *target_transform,
                    *camera_transform,
                    camera,
                );
                if is_facing_target {
                    interaction_opportunity.0.replace(target_entity);
                }
            }
        }
    }
    Ok(())
}

fn unpack_event(collision: &Collision) -> (Entity, Entity, bool) {
    let contacts = &collision.0;
    (
        contacts.entity1,
        contacts.entity2,
        contacts.during_current_frame,
    )
}

fn get_target_entity(
    player_query: &Query<Entity, With<Player>>,
    entity_a: Entity,
    entity_b: Entity,
) -> Option<Entity> {
    if player_query.get(entity_a).is_ok() {
        Some(entity_b)
    } else if player_query.get(entity_b).is_ok() {
        Some(entity_a)
    } else {
        None
    }
}

fn is_facing_target(
    player_transform: Transform,
    target_transform: Transform,
    camera_transform: Transform,
    camera: &IngameCamera,
) -> bool {
    if camera.kind == IngameCameraKind::FixedAngle {
        return true;
    }
    let camera_to_player = camera_transform.forward();
    let player_to_target = target_transform.translation - player_transform.translation;
    let angle = camera_to_player.angle_between(player_to_target);
    angle < TAU / 8.
}

#[sysfail(log(level = "error"))]
fn display_interaction_prompt(
    interaction_opportunity: Res<InteractionOpportunity>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut egui_contexts: EguiContexts,
    actions: Query<&ActionState<PlayerAction>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    dialog_target_query: Query<&DialogTarget>,
    mut freeze: ResMut<ActionsFrozen>,
) -> Result<()> {
    let Some(dialog_target) = interaction_opportunity
        .0
        .and_then(|e| dialog_target_query.get(e).ok())
    else {
        return Ok(());
    };
    for actions in actions.iter() {
        let window = primary_windows
            .get_single()
            .context("Failed to get primary window")?;
        egui::Window::new("Interaction")
            .collapsible(false)
            .title_bar(false)
            .auto_sized()
            .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
            .show(egui_contexts.ctx_mut(), |ui| {
                ui.label("E: Talk");
            });
        if actions.just_pressed(PlayerAction::Interact) {
            let mut dialogue_runner = dialogue_runner.single_mut();
            dialogue_runner.start_node(&dialog_target.node);
            freeze.freeze();
        }
    }
    Ok(())
}
