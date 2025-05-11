//! Setup methods for specific props that require additional logic or need to be initialized with fine-tuned constants.

use bevy::prelude::*;

mod burning_logs;
mod chair;
mod crate_;
mod lamp_plain;
mod lamp_shaded;
mod lamp_sitting;
mod lamp_wall_electric;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        burning_logs::plugin,
        chair::plugin,
        crate_::plugin,
        lamp_sitting::plugin,
        lamp_wall_electric::plugin,
        lamp_shaded::plugin,
        lamp_plain::plugin,
    ));
}
