use crate::movement::general_movement::Model;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct AnimationEntityLink(pub(crate) Entity);

impl Default for AnimationEntityLink {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

pub(crate) fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    models: Query<&Model>,
    mut commands: Commands,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("link_animations").entered();
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Multiple `AnimationPlayer`s are ambiguous for the same top parent");
        } else {
            let link_target = if let Ok(model) = models.get(top_entity) {
                model.target
            } else {
                top_entity
            };
            commands
                .entity(link_target)
                .insert(AnimationEntityLink(entity));
        }
    }
}

fn get_top_parent(curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    parent_query
        .iter_ancestors(curr_entity)
        .last()
        .unwrap_or(curr_entity)
}
