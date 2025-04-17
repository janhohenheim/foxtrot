use std::f32::consts::PI;

use animation::{NpcAnimationState, setup_npc_animations};
use assets::NpcAssets;
use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_tnua::{TnuaAnimatingState, prelude::*};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;

use crate::third_party::{bevy_trenchbroom::fix_gltf_rotation, bevy_yarnspinner::YarnNode};

use super::animation::AnimationPlayerAncestor;
mod ai;
mod animation;
mod assets;
mod sound;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ai::plugin, animation::plugin, assets::plugin, sound::plugin));
    app.register_type::<Npc>();
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
#[base(Transform, Visibility)]
#[model("models/fox/Fox.gltf")]
#[component(on_add = Self::on_add)]
// In Wasm, TrenchBroom classes are not automatically registered.
// So, we need to manually register the class in `src/third_party/bevy_trenchbroom/mod.rs`.
pub(crate) struct Npc;

const NPC_RADIUS: f32 = 0.8;
const NPC_FLOAT_HEIGHT: f32 = 1.0;

impl Npc {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        if world.is_scene_world() {
            return;
        }
        let model = world.resource::<NpcAssets>().model.clone();
        world
            .commands()
            .entity(entity)
            .queue(fix_gltf_rotation)
            .insert((
                Npc,
                TransformInterpolation,
                Collider::capsule(NPC_RADIUS, 0.1),
                TnuaController::default(),
                TnuaAvian3dSensorShape(Collider::cylinder(NPC_RADIUS - 0.01, 0.0)),
                ColliderDensity(200.0),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                TnuaAnimatingState::<NpcAnimationState>::default(),
                AnimationPlayerAncestor,
                YarnNode::new("Npc"),
            ))
            .with_child((
                Name::new("Npc Model"),
                SceneRoot(model),
                Transform::from_xyz(0.0, -NPC_FLOAT_HEIGHT, 0.0)
                    .with_rotation(Quat::from_rotation_y(PI)),
            ))
            .observe(setup_npc_animations);
    }
}
