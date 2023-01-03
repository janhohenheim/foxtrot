use crate::actions::ActionsFrozen;
use crate::dialog::{DialogEvent, DialogTarget};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use crate::player::PlayerSensor;
use crate::GameState;

pub struct InteractionsUi;

impl Plugin for InteractionsUi {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionUi>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_interaction_possibilities)
                .with_system(display_interaction_prompt),
        );
    }
}

#[derive(Resource, Default, Debug)]
pub struct InteractionUi(Option<InteractionKind>);

#[derive(Debug)]
pub enum InteractionKind {
    Dialog(DialogTarget),
}

fn update_interaction_possibilities(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<PlayerSensor>>,
    dialog_target_query: Query<&DialogTarget>,
    mut interaction_ui: ResMut<InteractionUi>,
    actions_frozen: Option<Res<ActionsFrozen>>,
) {
    if actions_frozen.is_some() {
        return;
    }
    for event in collision_events.iter() {
        let (entity_a, entity_b, kind, ongoing) = match event {
            CollisionEvent::Started(entity_a, entity_b, kind) => (entity_a, entity_b, kind, true),
            CollisionEvent::Stopped(entity_a, entity_b, kind) => (entity_a, entity_b, kind, false),
        };
        if *kind != CollisionEventFlags::SENSOR {
            continue;
        }
        let player = [player_query.get(*entity_a), player_query.get(*entity_b)]
            .into_iter()
            .filter_map(|res| res.ok())
            .next();
        let dialog_target = [
            dialog_target_query.get(*entity_a),
            dialog_target_query.get(*entity_b),
        ]
        .into_iter()
        .filter_map(|res| res.ok())
        .next();

        if let Some(dialog_target) = dialog_target && player.is_some(){
            if ongoing && interaction_ui.0.is_none(){
                interaction_ui.0 = Some(InteractionKind::Dialog(dialog_target.clone()))
            } else if let Some(interaction_kind) = &interaction_ui.0 &&
                let InteractionKind::Dialog(current_dialog_target) = interaction_kind &&
                *current_dialog_target == *dialog_target &&
                !ongoing {
                interaction_ui.0 = None;
            }
        }
    }
}

fn display_interaction_prompt(
    interaction_ui: Res<InteractionUi>,
    mut dialog_event_writer: EventWriter<DialogEvent>,
    mut egui_context: ResMut<EguiContext>,
) {
    let dialog_id = match &interaction_ui.0 {
        Some(InteractionKind::Dialog(dialog_id)) => dialog_id,
        _ => return,
    };
    egui::Window::new("Interaction")
        .fixed_size((300., 300.))
        .collapsible(false)
        .fixed_pos((300., 300.))
        .show(egui_context.ctx_mut(), |ui| {
            if ui.button("Talk").clicked() {
                dialog_event_writer.send(DialogEvent(dialog_id.dialog_id.clone()));
            }
        });
}
