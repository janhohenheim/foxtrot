use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct Counter(HashMap<String, usize>);

impl Counter {
    pub fn next(&mut self, name: &str) -> usize {
        *self
            .0
            .entry(name.to_owned())
            .and_modify(|count| *count += 1)
            .or_insert(1)
    }

    pub fn set_at_least(&mut self, name: &str, count: usize) {
        self.0
            .entry(name.to_owned())
            .and_modify(|current| *current = (*current).max(count))
            .or_insert(count);
    }
}
