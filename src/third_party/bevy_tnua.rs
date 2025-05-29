//! [Tnua](https://github.com/idanarye/bevy-tnua) powers our character controllers.

use avian3d::prelude::PhysicsSchedule;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::new(PhysicsSchedule),
        TnuaAvian3dPlugin::new(PhysicsSchedule),
    ));
}
