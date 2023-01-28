use crate::player_control::actions::Actions;
use crate::player_control::player_embodiment::Player;
use crate::util::math::{get_rotation_matrix_around_vector, get_rotation_matrix_around_y_axis};
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::na::Vector3;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

const MAX_DISTANCE: f32 = 10.0;

pub struct CameraPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct UiCamera;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PlayerCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiCamera>()
            .register_type::<PlayerCamera>()
            .add_startup_system(spawn_ui_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(despawn_ui_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_camera_controls.label("handle_camera_controls"))
                    .with_system(
                        keep_line_of_sight
                            .label("keep_line_of_sight")
                            .after("handle_camera_controls"),
                    )
                    .with_system(face_target.label("face_target").after("keep_line_of_sight"))
                    .with_system(cursor_grab_system),
            );
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), UiCamera, Name::new("Camera")));
}

fn despawn_ui_camera(mut commands: Commands, query: Query<Entity, With<UiCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_camera_controls(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    actions: Res<Actions>,
) {
    let dt = time.delta_seconds();
    let mouse_sensitivity = 0.5;
    let mut camera = match camera_query.iter_mut().next() {
        Some(transform) => transform,
        None => return,
    };
    let camera_movement = match actions.camera_movement {
        Some(vector) => vector * mouse_sensitivity * dt,
        None => return,
    };

    let direction = -camera.translation.try_normalize().unwrap_or(Vect::Z);
    let horizontal_rotation_axis = direction.xz().perp();
    let horizontal_rotation_axis =
        Vector3::new(horizontal_rotation_axis.x, 0., horizontal_rotation_axis.y);
    let horizontal_angle = camera_movement.x;
    let vertical_angle = -camera_movement.y;
    let vertical_angle = clamp_vertical_rotation(direction, vertical_angle);

    let horizontal_rotation_matrix = get_rotation_matrix_around_y_axis(horizontal_angle);
    let vertical_rotation_matrix =
        get_rotation_matrix_around_vector(vertical_angle, horizontal_rotation_axis);

    let rotated_direction: Vec3 =
        (vertical_rotation_matrix * horizontal_rotation_matrix * Vector3::from(direction)).into();
    camera.translation = -rotated_direction * MAX_DISTANCE;
}

fn face_target(mut camera_query: Query<&mut Transform, With<PlayerCamera>>) {
    for mut camera in &mut camera_query {
        camera.look_at(Vect::ZERO, Vect::Y);
    }
}

fn keep_line_of_sight(
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player_query: Query<&GlobalTransform, (With<Player>, Without<PlayerCamera>)>,
    rapier_context: Res<RapierContext>,
) {
    let mut camera = match camera_query.iter_mut().next() {
        Some(transform) => transform,
        None => return,
    };
    let player = match player_query.iter_mut().next() {
        Some(transform) => transform.translation(),
        None => return,
    };
    // camera.translation is the direction because it is a child of the entity with `Player`.
    // Thus, its `Transform` is relative to the player's, which makes it a direction.
    let location =
        get_raycast_location(&player, &camera.translation, &rapier_context, MAX_DISTANCE);

    camera.translation = location;
}

pub fn get_raycast_location(
    origin: &Vec3,
    direction: &Vec3,
    rapier_context: &Res<RapierContext>,
    max_distance: f32,
) -> Vec3 {
    let direction = direction.try_normalize().unwrap_or(Vect::Z);
    let max_toi = max_distance;
    let solid = true;
    let mut filter = QueryFilter::only_fixed();
    filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

    let min_distance_to_objects = 0.001;
    let distance = rapier_context
        .cast_ray(*origin, direction, max_toi, solid, filter)
        .map(|(_entity, toi)| toi - min_distance_to_objects)
        .unwrap_or(max_distance);

    direction * distance
}

fn clamp_vertical_rotation(current_direction: Vec3, angle: f32) -> f32 {
    let current_angle = current_direction.angle_between(Vect::Y);
    let new_angle = current_angle - angle;

    let angle_from_extremes = TAU / 32.;
    let max_angle = TAU / 2.0 - angle_from_extremes;
    let min_angle = 0.0 + angle_from_extremes;

    let clamped_angle = if new_angle > max_angle {
        max_angle - current_angle
    } else if new_angle < min_angle {
        min_angle - current_angle
    } else {
        angle
    };

    if clamped_angle.abs() < 0.01 {
        // This smooths user experience
        0.
    } else {
        clamped_angle
    }
}

fn cursor_grab_system(mut windows: ResMut<Windows>, key: Res<Input<KeyCode>>) {
    let window = windows.get_primary_mut().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        if matches!(window.cursor_grab_mode(), CursorGrabMode::None) {
            // if you want to use the cursor, but not let it leave the window,
            // use `Confined` mode:
            window.set_cursor_grab_mode(CursorGrabMode::Confined);

            // for a game that doesn't use the cursor (like a shooter):
            // use `Locked` mode to keep the cursor in one place
            window.set_cursor_grab_mode(CursorGrabMode::Locked);
            // also hide the cursor
            window.set_cursor_visibility(false);
        } else {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}
