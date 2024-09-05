use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerCamera>();
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub follow: Transform,
    pub offset: Transform,
    pub look_at: Option<Vec3>,
}

impl PlayerCamera {
    pub fn transform(self) -> Transform {
        self.follow * self.offset
    }
}

pub fn spawn_player_camera(world: &mut World) {
    world.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
        PlayerCamera::default(),
        IsDefaultUiCamera,
        //create_camera_action_input_manager_bundle(),
    ));
}
