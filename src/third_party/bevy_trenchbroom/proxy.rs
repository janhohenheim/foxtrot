//! `struct`s that mirror Bevy's builtin components so that they can be used in the level editor.

use bevy::prelude::{PointLight as BevyPointLight, *};
use bevy_trenchbroom::{config::TrenchBroomConfig, prelude::PointClass};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) trait RegisterProxies {
    fn register_proxies(self) -> TrenchBroomConfig;
}

impl RegisterProxies for TrenchBroomConfig {
    fn register_proxies(self) -> TrenchBroomConfig {
        self.register_class::<PointLight>()
    }
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
#[reflect(Component, Default, Debug)]
struct PointLight;
