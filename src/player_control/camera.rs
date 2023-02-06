use crate::player_control::actions::{Actions, ActionsFrozen};
use crate::util::math::{get_rotation_matrix_around_vector, get_rotation_matrix_around_y_axis};
use crate::util::trait_extension::{Vec2Ext, Vec3Ext};
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

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiCamera>()
            .register_type::<MainCamera>()
            .add_startup_system(spawn_ui_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(despawn_ui_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_camera_controls.label("handle_camera_controls"))
                    .with_system(
                        follow_target
                            .label("follow_target")
                            .after("handle_camera_controls"),
                    )
                    .with_system(face_target.label("face_target").after("follow_target"))
                    .with_system(
                        update_camera_transform
                            .label("update_camera_transform")
                            .after("face_target"),
                    )
                    .with_system(cursor_grab_system),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct UiCamera;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct MainCamera {
    pub current: CameraPosition,
    pub new: CameraPosition,
}

impl MainCamera {
    pub fn look_at(&mut self, target: Vec3) -> &mut Self {
        self.new.target = target;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CameraPosition {
    pub eye: Transform,
    pub target: Vec3,
}

impl CameraPosition {
    pub fn direction(&self) -> Option<Vec3> {
        (self.target - self.eye.translation).try_normalize()
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

fn handle_camera_controls(mut camera_query: Query<&mut MainCamera>, actions: Res<Actions>) {
    let mouse_sensitivity = 0.5;
    let camera_movement = match actions.camera_movement {
        Some(vector) => vector * mouse_sensitivity,
        None => return,
    };

    if camera_movement.is_approx_zero() {
        return;
    }
    for mut camera in camera_query.iter_mut() {
        let direction = camera.new.direction().unwrap_or(Vec3::Z);
        let horizontal_rotation_axis = direction.xz().perp().x0y();
        let horizontal_angle = camera_movement.x;
        let vertical_angle = -camera_movement.y;
        let vertical_angle = clamp_vertical_rotation(direction, vertical_angle);

        let horizontal_rotation_matrix = get_rotation_matrix_around_y_axis(horizontal_angle);
        let vertical_rotation_matrix =
            get_rotation_matrix_around_vector(vertical_angle, horizontal_rotation_axis.into());

        let rotated_direction: Vec3 =
            (vertical_rotation_matrix * horizontal_rotation_matrix * Vector3::from(direction))
                .into();

        camera.new.eye.translation = camera.new.target - rotated_direction * MAX_DISTANCE;
    }
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

fn follow_target(mut camera_query: Query<&mut MainCamera>) {
    for mut camera in &mut camera_query {
        let target_movement = camera.new.target - camera.current.target;
        camera.new.eye.translation = camera.current.eye.translation + target_movement;
    }
}

fn face_target(mut camera_query: Query<&mut MainCamera>) {
    for mut camera in &mut camera_query {
        let target = camera.new.target;
        if (target - camera.new.eye.translation).is_approx_zero() {
            continue;
        }
        camera.new.eye.look_at(target, Vect::Y);
    }
}

fn update_camera_transform(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
    rapier_context: Res<RapierContext>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut camera) in camera_query.iter_mut() {
        let scale = (2. * dt).max(1.);
        let line_of_sight_result = keep_line_of_sight(&camera, &rapier_context);
        if line_of_sight_result.correction == LineOfSightCorrection::Closer {
            transform.translation = line_of_sight_result.location;
        } else {
            let direction = line_of_sight_result.location - transform.translation;
            transform.translation += direction * scale;
        }
        transform.rotation = transform.rotation.slerp(camera.new.eye.rotation, scale);

        camera.current = camera.new.clone();
    }
}

fn keep_line_of_sight(camera: &MainCamera, rapier_context: &RapierContext) -> LineOfSightResult {
    let origin = camera.new.target;
    let desired_direction = camera.new.eye.translation - camera.new.target;
    let norm_direction = desired_direction.try_normalize().unwrap_or(Vec3::Z);

    let distance = get_raycast_distance(origin, norm_direction, rapier_context, MAX_DISTANCE);
    let location = origin + norm_direction * distance;
    let correction = if distance * distance < desired_direction.length_squared() {
        LineOfSightCorrection::Closer
    } else {
        LineOfSightCorrection::Further
    };
    LineOfSightResult {
        location,
        correction,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineOfSightResult {
    pub location: Vec3,
    pub correction: LineOfSightCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineOfSightCorrection {
    Closer,
    Further,
}

pub fn get_raycast_distance(
    origin: Vec3,
    direction: Vec3,
    rapier_context: &RapierContext,
    max_distance: f32,
) -> f32 {
    let max_toi = max_distance;
    let solid = true;
    let mut filter = QueryFilter::only_fixed();
    filter.flags |= QueryFilterFlags::EXCLUDE_SENSORS;

    let min_distance_to_objects = 0.001;
    rapier_context
        .cast_ray(origin, direction, max_toi, solid, filter)
        .map(|(_entity, toi)| toi - min_distance_to_objects)
        .unwrap_or(max_distance)
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    key: Res<Input<KeyCode>>,
    frozen: Option<Res<ActionsFrozen>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if frozen.is_some() {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
        return;
    }
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
