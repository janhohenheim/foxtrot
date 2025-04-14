use bevy::prelude::*;

mod candle;
pub(crate) use candle::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((candle::plugin,));
}
