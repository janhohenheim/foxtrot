use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Sun;

pub(crate) fn spawn(
    sun: Query<&Children, Added<Sun>>,
    mut directional_lights: Query<&mut DirectionalLight>,
) {
    for children in sun.iter() {
        for child in children.iter() {
            if let Ok(mut light) = directional_lights.get_mut(*child) {
                light.shadows_enabled = true;
            }
        }
    }
}
