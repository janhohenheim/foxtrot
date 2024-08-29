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
    #[default]
    Terrain,
    Player,
    Character,
    Prop,
    DialogSensor,
}

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
            CollisionLayerPreset::Player => CollisionLayers::new(
                [Player, Character],
                [Terrain, Character, Prop, DialogSensor],
            ),
            CollisionLayerPreset::Npc => {
                CollisionLayers::new([Character], [Terrain, Character, Prop])
            }
            // Since in our example our only sen
            CollisionLayerPreset::DialogSensor => CollisionLayers::new(DialogSensor, Player),
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
