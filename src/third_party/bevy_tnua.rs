use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{TnuaNotPlatform, prelude::*};
use bevy_tnua_avian3d::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));
}
