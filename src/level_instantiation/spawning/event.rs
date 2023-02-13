use crate::level_instantiation::spawning::GameObject;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Component, Clone, PartialEq, Default, Reflect, FromReflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub transform: Transform,
}

#[derive(
    Debug,
    Component,
    Resource,
    Clone,
    PartialEq,
    Default,
    Reflect,
    FromReflect,
    Serialize,
    Deserialize,
)]
#[reflect(Component, Resource, Serialize, Deserialize)]
pub struct DelayedSpawnEvent {
    pub tick_delay: usize,
    pub event: SpawnEvent,
}

impl DelayedSpawnEvent {
    pub fn pass_tick(&mut self) -> &mut Self {
        self.tick_delay = self.tick_delay.saturating_sub(1);
        self
    }
    pub fn is_done(&self) -> bool {
        self.tick_delay == 0
    }
}
