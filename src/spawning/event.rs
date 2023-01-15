use crate::spawning::GameObject;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ParentChangeEvent {
    pub name: Cow<'static, str>,
    pub new_parent: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct DuplicationEvent {
    pub name: Cow<'static, str>,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub transform: Transform,
    #[serde(default)]
    pub parent: Option<Cow<'static, str>>,
    #[serde(default)]
    pub name: Option<Cow<'static, str>>,
}

impl SpawnEvent {
    pub fn get_name_or_default(&self) -> String {
        self.name
            .clone()
            .map(|name| name.to_string())
            .unwrap_or_else(|| format!("{:?}", self.object))
    }
}
