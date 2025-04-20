//! [Hanabi](https://github.com/djeedai/bevy_hanabi) is our GPU particle system. Note that we don't use Hanabi
//! on Wasm as it is not supported on WebGL. If we only target WebGPU, we can safely reactivate it.

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(HanabiPlugin);
}
