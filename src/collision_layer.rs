use avian3d::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CollisionLayerPreset>();
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    /// Any [`RigidBody::Static`] collider that does not have an explicit layer will default to `Terrain`.
    /// This is used for e.g. the ground, trees, benches, buildings, etc.
    #[default]
    Terrain,
    /// A character is any entity that can move around and interact with the environment.
    Character,
    /// The player character.
    Player,
    /// A prop is a [`RigidBody::Dynamic`] collider, i.e. it can be influenced by physics.
    Prop,
    /// A [`Sensor`] that detects the presence of a player in order to allow the player to initiate dialogue.
    DialogSensor,
}

/// A way to easily override the default collision layer in Blender.
/// Does not include `Terrain` because that one is already the default.
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, PartialEq, Component)]
pub enum CollisionLayerPreset {
    Player,
    Npc,
    DialogSensor,
    Prop,
}

impl From<CollisionLayerPreset> for CollisionLayers {
    fn from(preset: CollisionLayerPreset) -> Self {
        use CollisionLayer::*;
        match preset {
            // A player is both a character and a player. They can interact pretty much everything.
            CollisionLayerPreset::Player => CollisionLayers::new(
                [Player, Character],
                [Terrain, Character, Prop, DialogSensor],
            ),
            // NPCs are just characters. They cannot interact with sensors in this case.
            CollisionLayerPreset::Npc => {
                CollisionLayers::new([Character], [Terrain, Character, Prop])
            }
            // Dialog sensors can only interact with players.
            CollisionLayerPreset::DialogSensor => CollisionLayers::new(DialogSensor, Player),
            // Props can interact with terrain, other props, and characters.
            CollisionLayerPreset::Prop => CollisionLayers::new(Prop, [Terrain, Prop, Character]),
        }
    }
}

impl Component for CollisionLayerPreset {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let preset = *world.get::<CollisionLayerPreset>(entity).unwrap();
            let mut commands = world.commands();
            commands
                .entity(entity)
                .insert(CollisionLayers::from(preset));
        });
    }
}
