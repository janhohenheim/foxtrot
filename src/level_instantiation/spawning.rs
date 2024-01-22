use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::level_instantiation::spawning::despawn::{despawn, Despawn};
use crate::level_instantiation::spawning::post_spawn_modification::{
    despawn_removed, set_hidden, set_shadows,
};
use crate::GameState;
pub(crate) use animation_link::AnimationEntityLink;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use spew::prelude::*;
use strum_macros::EnumIter;

mod animation_link;
mod despawn;
pub(crate) mod objects;
mod post_spawn_modification;

pub(crate) fn spawning_plugin(app: &mut App) {
    app.add_plugins(SpewPlugin::<GameObject, Transform>::default())
        .register_type::<Despawn>()
        .register_type::<AnimationEntityLink>()
        .add_spawners((
            (GameObject::Empty, objects::primitives::spawn_empty),
            (GameObject::Box, objects::primitives::spawn_box),
            (GameObject::Triangle, objects::primitives::spawn_triangle),
            (GameObject::Sphere, objects::primitives::spawn_sphere),
            (GameObject::Capsule, objects::primitives::spawn_capsule),
            (GameObject::Sunlight, objects::sunlight::spawn),
            (GameObject::PointLight, objects::point_light::spawn),
            (GameObject::Npc, objects::npc::spawn),
            (GameObject::Player, objects::player::spawn),
            (GameObject::Level, objects::level::spawn),
            (GameObject::Orb, objects::orb::spawn),
            (GameObject::Camera, objects::camera::spawn),
            (GameObject::Skydome, objects::skydome::spawn),
        ))
        .add_systems(
            Update,
            (despawn, link_animations).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (set_hidden, despawn_removed, set_shadows).run_if(in_state(GameState::Playing)),
        );
}

#[derive(
    Debug,
    EnumIter,
    Component,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Reflect,
    Serialize,
    Deserialize,
    Default,
)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) enum GameObject {
    #[default]
    Empty,
    Box,
    Triangle,
    Sphere,
    Capsule,
    Sunlight,
    PointLight,
    Npc,
    Player,
    Level,
    Orb,
    Camera,
    Skydome,
}
