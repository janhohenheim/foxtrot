use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use leafwing_input_manager::prelude::*;

pub(crate) fn update_kind(
    mut camera_query: Query<(&mut IngameCamera, &ActionState<CameraAction>)>,
    config: Res<GameConfig>,
) {
    for (mut camera, actions) in camera_query.iter_mut() {
        let zoom = actions.clamped_value(CameraAction::Zoom);
        let zoomed_out = zoom < -1e-5;
        let zoomed_in = zoom > 1e-5;
        let new_kind = match camera.kind {
            IngameCameraKind::FirstPerson if zoomed_out => Some(IngameCameraKind::ThirdPerson),
            IngameCameraKind::ThirdPerson
                if camera.desired_distance < config.camera.third_person.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::FirstPerson)
            }
            IngameCameraKind::ThirdPerson
                if camera.desired_distance > config.camera.third_person.max_distance - 1e-5
                    && zoomed_out =>
            {
                Some(IngameCameraKind::FixedAngle)
            }
            IngameCameraKind::FixedAngle
                if camera.desired_distance < config.camera.fixed_angle.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::ThirdPerson)
            }
            _ => None,
        };
        if let Some(new_kind) = new_kind {
            camera.kind = new_kind;
        }
    }
}

pub(crate) fn update_drivers(mut camera_query: Query<(&IngameCamera, &mut Rig)>) {
    for (camera, mut rig) in camera_query.iter_mut() {
        match camera.kind {
            IngameCameraKind::ThirdPerson => set_third_person_drivers(&mut rig),
            IngameCameraKind::FirstPerson => match camera.secondary_target {
                Some(_) => set_first_person_drivers_with_target(&mut rig),
                None => set_first_person_drivers_without_target(&mut rig),
            },
            IngameCameraKind::FixedAngle => set_fixed_angle_drivers(&mut rig),
        };
    }
}

fn set_third_person_drivers(rig: &mut Rig) {
    rig.ensure_driver_exists(Arm::new(default()));
    // Overriding because tracking_predictive cannot be changed after creation.
    rig.override_driver(LookAt::new(default()).tracking_predictive(true));
}

fn set_first_person_drivers_without_target(rig: &mut Rig) {
    rig.remove_driver::<LookAt>();
    rig.remove_driver::<Arm>();
}

fn set_first_person_drivers_with_target(rig: &mut Rig) {
    rig.remove_driver::<Arm>();
    rig.override_driver(LookAt::new(default()));
}

fn set_fixed_angle_drivers(rig: &mut Rig) {
    rig.ensure_driver_exists(Arm::new(default()));
    rig.remove_driver::<LookAt>();
}

trait RigExt {
    fn remove_driver<T: RigDriverTraits>(&mut self);
    fn ensure_driver_exists<T: RigDriverTraits>(&mut self, driver: T);
    fn override_driver<T: RigDriverTraits>(&mut self, driver: T) {
        self.remove_driver::<T>();
        self.ensure_driver_exists(driver);
    }
}

impl RigExt for Rig {
    fn remove_driver<T: RigDriverTraits>(&mut self) {
        let index = self
            .drivers
            .iter()
            .position(|driver| driver.as_ref().as_any().downcast_ref::<T>().is_some());
        if let Some(index) = index {
            self.drivers.remove(index);
        }
    }

    fn ensure_driver_exists<T: RigDriverTraits>(&mut self, driver: T) {
        if self.try_driver::<T>().is_none() {
            self.drivers.push(Box::new(driver));
        }
    }
}
