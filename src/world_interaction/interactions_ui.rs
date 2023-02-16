use crate::player_control::actions::{Actions, ActionsFrozen};
use crate::world_interaction::dialog::{DialogEvent, DialogTarget};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use std::f32::consts::TAU;

use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use crate::player_control::player_embodiment::Player;
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};

pub struct InteractionsUiPlugin;

impl Plugin for InteractionsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_interaction_possibilities.pipe(log_errors))
                .with_system(display_interaction_prompt.pipe(log_errors)),
        );
    }
}

#[derive(Resource, Debug)]
pub struct InteractionUi {
    source: Option<Entity>,
    kind: InteractionKind,
}

#[derive(Debug)]
pub enum InteractionKind {
    Dialog(DialogTarget),
}

fn update_interaction_possibilities(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    dialog_target_query: Query<(Entity, &DialogTarget, &Transform), Without<Player>>,
    parent_query: Query<&Parent>,
    interaction_ui: Option<Res<InteractionUi>>,
    camera_query: Query<&IngameCamera>,
) -> Result<()> {
    for event in collision_events.iter() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);

        let (player_entity, target_entity) =
            match determine_player_and_target(&player_query, &parent_query, *entity_a, *entity_b) {
                Some((dialog_source, dialog_target)) => (dialog_source, dialog_target),
                None => continue,
            };

        if let Ok((dialog_source, dialog_target, dialog_target_transform)) = dialog_target_query.get(target_entity) &&
            let Ok((_, player_transform)) = player_query.get(player_entity) {
            let camera = camera_query.single();
            let is_facing_target = is_facing_target(
                *player_transform,
                *dialog_target_transform,
                camera,
            );
            if ongoing && interaction_ui.is_none() && is_facing_target {
                commands.insert_resource::<InteractionUi>(InteractionUi {
                    source: Some(dialog_source),
                    kind: InteractionKind::Dialog(dialog_target.clone()),
                });
            } else if let Some(interaction_ui) = &interaction_ui &&
                let InteractionKind::Dialog(current_dialog_target) = &interaction_ui.kind &&
                *current_dialog_target == *dialog_target &&
                !ongoing {
                commands.remove_resource::<InteractionUi>();
            }
        }
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
    player_query: &Query<(Entity, &Transform), With<Player>>,
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
    info!("Angle: {}", angle);
    angle < TAU / 4.
}

fn display_interaction_prompt(
    interaction_ui: Option<Res<InteractionUi>>,
    mut dialog_event_writer: EventWriter<DialogEvent>,
    mut egui_context: ResMut<EguiContext>,
    actions: Res<Actions>,
    windows: Res<Windows>,
    actions_frozen: Res<ActionsFrozen>,
) -> Result<()> {
    if actions_frozen.is_frozen() {
        return Ok(());
    }
    let interaction_ui = match interaction_ui {
        Some(interaction_ui) => interaction_ui,
        None => return Ok(()),
    };

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
    if actions.player.interact {
        match &interaction_ui.kind {
            InteractionKind::Dialog(dialog_target) => {
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
