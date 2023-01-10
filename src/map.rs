use bevy::prelude::*;

use crate::loading::DynamicSceneAssets;
use crate::spawning::{GameObject, SpawnEvent, SpawnEventSender};
use crate::GameState;
use std::f32::consts::TAU;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

const GRASS_SIZE: f32 = 10.;

fn load_scene(mut commands: Commands, scenes: Res<DynamicSceneAssets>) {
    commands.spawn(DynamicSceneBundle {
        scene: scenes.demo.clone(),
        ..default()
    });
}
fn setup(mut commands: Commands, mut spawner: EventWriter<SpawnEvent>) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.1,
    });

    let grass_x = 10;
    let grass_z = 10;

    for x in 0..grass_x {
        for z in 0..grass_z {
            SpawnEventSender::new(&mut spawner)
                .object(GameObject::Grass)
                .translation((
                    GRASS_SIZE * (-grass_x / 2 + x) as f32,
                    0.,
                    GRASS_SIZE * (-grass_z / 2 + z) as f32,
                ))
                .parent("Grass Container")
                .send();
        }
    }

    let wall_width = 3.;
    SpawnEventSender::new(&mut spawner)
        .parent("House")
        .object(GameObject::Doorway)
        .send()
        .object(GameObject::Wall)
        .translation((0., 0., wall_width))
        .send()
        .translation((0., 0., -wall_width))
        .send();

    SpawnEventSender::new(&mut spawner)
        .object(GameObject::Sunlight)
        .rotation(Quat::from_rotation_x(-TAU / 8.))
        .send();

    SpawnEventSender::new(&mut spawner)
        .object(GameObject::Npc)
        .transform(Transform {
            translation: Vec3::new(-5., 1., 0.),
            rotation: Quat::from_rotation_y(TAU / 4.),
            ..default()
        })
        .send();
}
