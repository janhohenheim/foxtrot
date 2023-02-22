use crate::player_control::actions::{ActionsFrozen, PlayerAction};
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::player_control::player_embodiment::Player;
use crate::util::log_error::log_errors;
use crate::world_interaction::dialog::{DialogEvent, DialogTarget};
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct InteractionsUiPlugin;

impl Plugin for InteractionsUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InteractionOpportunities>()
            .init_resource::<InteractionOpportunities>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_interaction_opportunities)
                    .with_system(
                        update_interaction_ui
                            .pipe(log_errors)
                            .after(update_interaction_opportunities),
                    )
                    .with_system(display_interaction_prompt.pipe(log_errors)),
            );
    }
}

#[derive(Resource, Debug)]
pub struct InteractionUi {
    source: Entity,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct InteractionOpportunities(pub HashSet<Entity>);

fn update_interaction_opportunities(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    parent_query: Query<&Parent>,
    mut interaction_opportunities: ResMut<InteractionOpportunities>,
) {
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);

        let (_player_entity, target_entity) =
            match determine_player_and_target(&player_query, &parent_query, entity_a, entity_b) {
                Some((dialog_source, dialog_target)) => (dialog_source, dialog_target),
                None => continue,
            };
        if ongoing {
            interaction_opportunities.0.insert(target_entity);
        } else {
            interaction_opportunities.0.remove(&target_entity);
        }
    }
}

fn update_interaction_ui(
    mut commands: Commands,
    interaction_ui: Option<ResMut<InteractionUi>>,
    non_player_query: Query<&Transform, Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    interaction_opportunities: Res<InteractionOpportunities>,
    camera_query: Query<&IngameCamera>,
) -> Result<()> {
    let mut valid_target = None;
    for entity in interaction_opportunities.0.iter() {
        let target_transform = non_player_query
            .get(*entity)
            .context("Failed to get transform of interaction target")?;
        for player_transform in player_query.iter() {
            for camera in camera_query.iter() {
                let is_facing_target =
                    is_facing_target(*player_transform, *target_transform, camera);
                if is_facing_target {
                    valid_target = Some(*entity);
                    break;
                }
            }
        }
    }
    if let Some(mut interaction_ui) = interaction_ui {
        if let Some(valid_target) = valid_target {
            interaction_ui.source = valid_target;
        } else {
            commands.remove_resource::<InteractionUi>();
        }
    } else if let Some(valid_target) = valid_target {
        commands.insert_resource(InteractionUi {
            source: valid_target,
        });
    }

    Ok(())
}

fn unpack_event(event: &CollisionEvent) -> (Entity, Entity, bool) {
    match event {
        CollisionEvent::Started(entity_a, entity_b, _kind) => (*entity_a, *entity_b, true),
        CollisionEvent::Stopped(entity_a, entity_b, _kind) => (*entity_a, *entity_b, false),
    }
}

fn determine_player_and_target(
    player_query: &Query<Entity, With<Player>>,
    parent_query: &Query<&Parent>,
    entity_a: Entity,
    entity_b: Entity,
) -> Option<(Entity, Entity)> {
    if player_query.get(entity_a).is_ok() {
        let player_entity = entity_a;
        let target_entity = parent_query
            .get(entity_b)
            .map(|parent| parent.get())
            .unwrap_or(entity_b);
        Some((player_entity, target_entity))
    } else if player_query.get(entity_b).is_ok() {
        let player_entity = entity_b;
        let target_entity = parent_query
            .get(entity_a)
            .map(|parent| parent.get())
            .unwrap_or(entity_a);
        Some((player_entity, target_entity))
    } else {
        None
    }
}

fn is_facing_target(
    player_transform: Transform,
    target_transform: Transform,
    camera: &IngameCamera,
) -> bool {
    if matches!(camera.kind, IngameCameraKind::FixedAngle(_)) {
        return true;
    }
    let camera_to_player = camera.forward();
    let player_to_target = target_transform.translation - player_transform.translation;
    let angle = camera_to_player.angle_between(player_to_target);
    angle < TAU / 8.
}

fn display_interaction_prompt(
    interaction_ui: Option<Res<InteractionUi>>,
    mut dialog_event_writer: EventWriter<DialogEvent>,
    mut egui_context: ResMut<EguiContext>,
    actions: Query<&ActionState<PlayerAction>>,
    windows: Res<Windows>,
    actions_frozen: Res<ActionsFrozen>,
    dialog_target_query: Query<&DialogTarget>,
) -> Result<()> {
    if actions_frozen.is_frozen() {
        return Ok(());
    }
    let interaction_ui = match interaction_ui {
        Some(interaction_ui) => interaction_ui,
        None => return Ok(()),
    };

    for actions in actions.iter() {
        let window = windows
            .get_primary()
            .context("Failed to get primary window")?;
        egui::Window::new("Interaction")
            .collapsible(false)
            .title_bar(false)
            .auto_sized()
            .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.label("E: Talk");
            });
        if actions.just_pressed(PlayerAction::Interact) {
            if let Ok(dialog_target) = dialog_target_query.get(interaction_ui.source) {
                dialog_event_writer.send(DialogEvent {
                    source: interaction_ui.source,
                    dialog: dialog_target.dialog_id.clone(),
                    page: None,
                });
            }
        }
    }
    Ok(())
}
