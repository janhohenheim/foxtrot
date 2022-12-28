use bevy::prelude::*;

use crate::player::Player;
use crate::GameState;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_plugin(LookTransformPlugin)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    let eye = Vec3::default();
    let target = Vec3::default();
    commands.spawn((
        LookTransformBundle {
            transform: LookTransform::new(eye, target),
            smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
        },
        Camera3dBundle::default(),
        Name::new("Camera"),
    ));
}

fn handle_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut LookTransform>,
) {
    let player = match player_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    for mut camera in &mut camera_query {
        let max_distance = 550.0;
        let direction = camera.look_direction().unwrap_or(Vect::Z);
        camera.target = player.translation;
        camera.eye = camera.target - direction * max_distance;
    }
}
