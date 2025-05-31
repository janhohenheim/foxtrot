//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    Pause,
    audio::{DEFAULT_VOLUME, max_volume},
    gameplay::player::camera::{CameraSensitivity, WorldModelFov},
    menus::Menu,
    screens::Screen,
    theme::{palette::SCREEN_BACKGROUND, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<VolumeSliderSettings>();
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<GlobalVolumeLabel>();
    app.add_systems(
        Update,
        (
            update_global_volume.run_if(resource_exists_and_changed::<VolumeSliderSettings>),
            update_volume_label,
            update_camera_sensitivity_label,
            update_camera_fov_label,
        )
            .run_if(in_state(Menu::Settings)),
    );
}

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_settings_menu(mut commands: Commands, paused: Res<State<Pause>>) {
    let mut entity_commands = commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Menu::Settings),
        GlobalZIndex(2),
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
                    // Audio
                    (
                        widget::label("Audio Volume"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(GlobalVolumeLabel, lower_volume, raise_volume),
                    // Camera Sensitivity
                    (
                        widget::label("Camera Sensitivity"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(
                        CameraSensitivityLabel,
                        lower_camera_sensitivity,
                        raise_camera_sensitivity
                    ),
                    // Camera FOV
                    (
                        widget::label("Camera FOV"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    widget::plus_minus_bar(CameraFovLabel, lower_camera_fov, raise_camera_fov),
                ],
            ),
            widget::button("Back", go_back_on_click),
        ],
    ));
    if paused.get() == &Pause(false) {
        entity_commands.insert(BackgroundColor(SCREEN_BACKGROUND));
    }
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

#[cfg_attr(feature = "hot_patch", hot)]
fn update_global_volume(
    mut global_volume: ResMut<GlobalVolume>,
    volume_step: Res<VolumeSliderSettings>,
) {
    global_volume.volume = volume_step.volume();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn lower_volume(_trigger: Trigger<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.decrement();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn raise_volume(_trigger: Trigger<Pointer<Click>>, mut volume_step: ResMut<VolumeSliderSettings>) {
    volume_step.increment();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[cfg_attr(feature = "hot_patch", hot)]
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

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSensitivityLabel;

#[cfg_attr(feature = "hot_patch", hot)]
fn lower_camera_sensitivity(
    _trigger: Trigger<Pointer<Click>>,
    mut camera_sensitivity: ResMut<CameraSensitivity>,
) {
    camera_sensitivity.0 -= 0.1;
    const MIN_SENSITIVITY: f32 = 0.1;
    camera_sensitivity.x = camera_sensitivity.x.max(MIN_SENSITIVITY);
    camera_sensitivity.y = camera_sensitivity.y.max(MIN_SENSITIVITY);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn raise_camera_sensitivity(
    _trigger: Trigger<Pointer<Click>>,
    mut camera_sensitivity: ResMut<CameraSensitivity>,
) {
    camera_sensitivity.0 += 0.1;
    const MAX_SENSITIVITY: f32 = 20.0;
    camera_sensitivity.x = camera_sensitivity.x.min(MAX_SENSITIVITY);
    camera_sensitivity.y = camera_sensitivity.y.min(MAX_SENSITIVITY);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn update_camera_sensitivity_label(
    mut label: Single<&mut Text, With<CameraSensitivityLabel>>,
    camera_sensitivity: Res<CameraSensitivity>,
) {
    label.0 = format!("{:.1}", camera_sensitivity.x);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraFovLabel;

fn lower_camera_fov(_trigger: Trigger<Pointer<Click>>, mut camera_fov: ResMut<WorldModelFov>) {
    camera_fov.0 -= 1.0;
    camera_fov.0 = camera_fov.0.max(45.0);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn raise_camera_fov(_trigger: Trigger<Pointer<Click>>, mut camera_fov: ResMut<WorldModelFov>) {
    camera_fov.0 += 1.0;
    camera_fov.0 = camera_fov.0.min(130.0);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn update_camera_fov_label(
    mut label: Single<&mut Text, With<CameraFovLabel>>,
    camera_fov: Res<WorldModelFov>,
) {
    label.0 = format!("{:.1}", camera_fov.0);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn go_back_on_click(
    _trigger: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

#[cfg_attr(feature = "hot_patch", hot)]
fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
