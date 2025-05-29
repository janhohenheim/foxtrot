//! [Tnua](https://github.com/idanarye/bevy-tnua) powers our character controllers.

use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));
}
