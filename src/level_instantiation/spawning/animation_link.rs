use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AnimationEntityLink(pub Entity);

impl Default for AnimationEntityLink {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("link_animations").entered();
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Multiple `AnimationPlayer`s are ambiguous for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity));
        }
    }
}

/// Source: <https://github.com/bevyengine/bevy/discussions/5564>
fn get_top_parent(curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    let mut last_entity = curr_entity;
    while let Ok(parent) = parent_query.get(last_entity) {
        last_entity = parent.get();
    }
    last_entity
}
