//! [Enhanced Input](https://github.com/projectharmonia/bevy_enhanced_input) is our input system.
//! It is slated to be upstreamed into Bevy itself at some point.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin);
}
