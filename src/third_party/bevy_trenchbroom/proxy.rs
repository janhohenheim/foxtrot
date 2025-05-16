//! `struct`s that mirror Bevy's builtin components so that they can be used in the level editor.

use bevy::prelude::{
    DirectionalLight as BevyDirectionalLight, PointLight as BevyPointLight,
    SpotLight as BevySpotLight, *,
};
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PointLight>();
    app.register_type::<DirectionalLight>();
    app.register_type::<SpotLight>();
    app.register_type::<FuncGroup>();
}

/// A light that emits light in all directions from a central point.
///
/// Real-world values for `intensity` (luminous power in lumens) based on the electrical power
/// consumption of the type of real-world light are:
///
/// | Luminous Power (lumen) (i.e. the intensity member) | Incandescent non-halogen (Watts) | Incandescent halogen (Watts) | Compact fluorescent (Watts) | LED (Watts) |
/// |------|-----|----|--------|-------|
/// | 200  | 25  |    | 3-5    | 3     |
/// | 450  | 40  | 29 | 9-11   | 5-8   |
/// | 800  | 60  |    | 13-15  | 8-12  |
/// | 1100 | 75  | 53 | 18-20  | 10-16 |
/// | 1600 | 100 | 72 | 24-28  | 14-17 |
/// | 2400 | 150 |    | 30-52  | 24-30 |
/// | 3100 | 200 |    | 49-75  | 32    |
/// | 4000 | 300 |    | 75-100 | 40.5  |
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/Lumen_(unit)#Lighting)
#[derive(PointClass, Component, Debug, Clone, Copy, Default, Reflect)]
#[base(BevyPointLight)]
#[iconsprite({ path: "images/point_light.png", scale: 0.1 })]
#[reflect(QuakeClass, Component, Default, Debug)]
#[classname("light_point")]
struct PointLight;

/// A Directional light.
///
/// Directional lights don't exist in reality but they are a good
/// approximation for light sources VERY far away, like the sun or
/// the moon.
///
/// The light shines along the forward direction of the entity's transform. With a default transform
/// this would be along the negative-Z axis.
///
/// Valid values for `illuminance` are:
///
/// | Illuminance (lux) | Surfaces illuminated by                        |
/// |-------------------|------------------------------------------------|
/// | 0.0001            | Moonless, overcast night sky (starlight)       |
/// | 0.002             | Moonless clear night sky with airglow          |
/// | 0.05–0.3          | Full moon on a clear night                     |
/// | 3.4               | Dark limit of civil twilight under a clear sky |
/// | 20–50             | Public areas with dark surroundings            |
/// | 50                | Family living room lights                      |
/// | 80                | Office building hallway/toilet lighting        |
/// | 100               | Very dark overcast day                         |
/// | 150               | Train station platforms                        |
/// | 320–500           | Office lighting                                |
/// | 400               | Sunrise or sunset on a clear day.              |
/// | 1000              | Overcast day; typical TV studio lighting       |
/// | 10,000–25,000     | Full daylight (not direct sun)                 |
/// | 32,000–100,000    | Direct sunlight                                |
///
/// Source: [Wikipedia](https://en.wikipedia.org/wiki/Lux)
///
/// ## Shadows
///
/// To enable shadows, set the `shadows_enabled` property to `true`.
///
/// Shadows are produced via [cascaded shadow maps](https://developer.download.nvidia.com/SDK/10.5/opengl/src/cascaded_shadow_maps/doc/cascaded_shadow_maps.pdf).
///
/// To modify the cascade setup, such as the number of cascades or the maximum shadow distance,
/// change the [`CascadeShadowConfig`](bevy::pbr::CascadeShadowConfig) component of the entity with the [`DirectionalLight`].
///
/// To control the resolution of the shadow maps, use the [`DirectionalLightShadowMap`](bevy::pbr::DirectionalLightShadowMap) resource.
#[derive(PointClass, Component, Debug, Clone, Copy, Default, Reflect)]
#[base(BevyDirectionalLight)]
#[iconsprite({ path: "images/point_light.png", scale: 0.1 })]
#[reflect(QuakeClass, Component, Default, Debug)]
#[classname("light_directional")]
struct DirectionalLight;

/// A light that emits light in a given direction from a central point.
///
/// Behaves like a point light in a perfectly absorbent housing that
/// shines light only in a given direction. The direction is taken from
/// the transform, and can be specified with [`Transform::looking_at`](Transform::looking_at).
#[derive(PointClass, Component, Debug, Clone, Copy, Default, Reflect)]
#[base(BevySpotLight)]
#[iconsprite({ path: "images/point_light.png", scale: 0.1 })]
#[reflect(QuakeClass, Component, Default, Debug)]
#[classname("light_spot")]
struct SpotLight;

#[derive(SolidClass, Component, Debug, Clone, Copy, Default, Reflect)]
#[base(Transform, Visibility)]
#[reflect(QuakeClass, Component, Default, Debug)]
#[spawn_hooks(SpawnHooks::new().convex_collider().smooth_by_default_angle())]
struct FuncGroup;
