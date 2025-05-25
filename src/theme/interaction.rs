use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{PostPhysicsAppSystems, asset_tracking::LoadResource, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.load_resource::<InteractionAssets>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            trigger_interaction_sound_effect,
        )
            .run_if(resource_exists::<InteractionAssets>)
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct InteractionPalette {
    pub(crate) none: Color,
    pub(crate) hovered: Color,
    pub(crate) pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub(crate) struct OnPress;

#[cfg_attr(feature = "hot_patch", hot)]
fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub(crate) struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    press: Handle<AudioSource>,
}

impl InteractionAssets {
    pub(crate) const PATH_BUTTON_HOVER: &'static str = "audio/sound_effects/button_hover.ogg";
    pub(crate) const PATH_BUTTON_PRESS: &'static str = "audio/sound_effects/button_press.ogg";
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load(Self::PATH_BUTTON_HOVER),
            press: assets.load(Self::PATH_BUTTON_PRESS),
        }
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn trigger_interaction_sound_effect(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    interaction_assets: Res<InteractionAssets>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => interaction_assets.hover.clone(),
            Interaction::Pressed => interaction_assets.press.clone(),
            _ => continue,
        };
        commands.spawn(sound_effect(source));
    }
}
