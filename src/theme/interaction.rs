use bevy::{picking::hover::Hovered, prelude::*, ui::Pressed};
use bevy_seedling::sample::{AudioSample, SamplePlayer};

use crate::{PostPhysicsAppSystems, asset_tracking::LoadResource, audio::SfxPool};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<InteractionAssets>();
    app.add_systems(
        Update,
        (apply_interaction_palette, trigger_interaction_sound_effect)
            .run_if(resource_exists::<InteractionAssets>)
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[require(Hovered)]
pub(crate) struct InteractionPalette {
    pub(crate) none: Color,
    pub(crate) hovered: Color,
    pub(crate) pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (
            Has<Pressed>,
            &Hovered,
            &InteractionPalette,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
) {
    for (pressed, Hovered(hovered), palette, mut background) in &mut palette_query {
        *background = match (pressed, hovered) {
            (true, _) => palette.pressed,
            (false, true) => palette.hovered,
            (false, false) => palette.none,
        }
        .into();
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub(crate) struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSample>,
    #[dependency]
    press: Handle<AudioSample>,
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

fn trigger_pressed_sound_effect(
    _: On<Pointer<Press>>,
    mut commands: Commands,
    interaction_assets: Res<InteractionAssets>,
) {
    commands.spawn((SamplePlayer::new(interaction_assets.press.clone()), SfxPool));
}

fn trigger_interaction_sound_effect(
    interaction_query: Populated<&Hovered, Changed<Hovered>>,
    interaction_assets: Res<InteractionAssets>,
    mut commands: Commands,
) {
    for _ in interaction_query.iter().filter(|h| h.0) {
        commands.spawn((SamplePlayer::new(interaction_assets.hover.clone()), SfxPool));
    }
}
