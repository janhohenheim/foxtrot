use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::SoundEffect};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.register_type::<InteractionAssets>();
    app.load_resource::<InteractionAssets>();
    app.add_systems(
        Update,
        (
            apply_interaction_palette,
            trigger_interaction_sound_effect.run_if(resource_exists::<InteractionAssets>),
        ),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    press: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            press: assets.load("audio/sound_effects/button_press.ogg"),
        }
    }
}

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
        commands.spawn((AudioPlayer(source), PlaybackSettings::DESPAWN, SoundEffect));
    }
}
