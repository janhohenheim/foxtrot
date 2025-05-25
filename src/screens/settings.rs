//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use bevy::{audio::Volume, prelude::*, ui::Val::*};

use crate::{
    audio::{DEFAULT_VOLUME, max_volume},
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<VolumeSliderSettings>();
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);

    app.register_type::<GlobalVolumeLabel>();
    app.add_systems(
        Update,
        (
            update_global_volume.run_if(resource_exists_and_changed::<VolumeSliderSettings>),
            update_volume_label,
        )
            .run_if(in_state(Screen::Settings)),
    );
}

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            (
                Name::new("Settings Grid"),
                Node {
                    display: Display::Grid,
                    row_gap: Px(10.0),
                    column_gap: Px(30.0),
                    grid_template_columns: RepeatedGridTrack::px(2, 400.0),
                    ..default()
                },
                children![
                    (
                        widget::label("Audio Volume"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    volume_widget(),
                ],
            ),
            widget::button("Back", enter_title_screen),
        ],
    ));
}

fn volume_widget() -> impl Bundle {
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_volume),
            widget::button_small("+", raise_volume),
            (
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
        ],
    )
}

#[derive(Resource, Reflect, Debug)]
struct VolumeSliderSettings(usize);

impl VolumeSliderSettings {
    fn increment(&mut self) {
        self.0 = Self::MAX_TICK_COUNT.min(self.0 + 1);
    }

    fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    fn volume(&self) -> Volume {
        let max_gain = max_volume().to_linear();
        let mid_gain = DEFAULT_VOLUME.to_linear();

        let t = self.0 as f32 / Self::MAX_TICK_COUNT as f32;
        let gain = Self::curved_interpolation(t, mid_gain, max_gain);
        Volume::Linear(gain)
    }

    /// Interpolates between 0, a, and b nonlinearly,
    /// such that t = 0 -> 0, t = 0.5 -> a, t = 1 -> b
    fn curved_interpolation(t: f32, a: f32, b: f32) -> f32 {
        if t <= 0.5 {
            let t2 = t / 0.5;
            a * (3.0 * t2.powi(2) - 2.0 * t2.powi(3))
        } else {
            let t2 = (t - 0.5) / 0.5;
            let smooth = 3.0 * t2.powi(2) - 2.0 * t2.powi(3);
            a + (b - a) * smooth
        }
    }

    /// How many ticks the volume slider supports
    const MAX_TICK_COUNT: usize = 20;
}

impl Default for VolumeSliderSettings {
    fn default() -> Self {
        Self(Self::MAX_TICK_COUNT / 2)
    }
}

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn update_global_volume(
    mut global_volume: ResMut<GlobalVolume>,
    volume_step: Res<VolumeSliderSettings>,
) {
    global_volume.volume = volume_step.volume();
}

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn lower_volume(_trigger: Trigger<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.decrement();
}

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn raise_volume(_trigger: Trigger<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.increment();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn update_volume_label(
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
    slider: Res<VolumeSliderSettings>,
) {
    let ticks = slider.0;
    let filled = "█".repeat(ticks);
    let empty = " ".repeat(VolumeSliderSettings::MAX_TICK_COUNT - ticks);
    let text = filled + &empty + "|";
    label.0 = text;
}

#[cfg_attr(feature = "hot_patch", bevy_simple_subsecond_system::hot)]
fn enter_title_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Title);
}
