use bevy::{
    input::mouse::*,
    prelude::*,
};
use crate::{
    player::Player,
    base::*,
    camera::*,
};

#[derive(Default)]
pub struct InputState {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut MovementData, &RotationData)>,
    mut camera_query: Query<(&Camera, &mut MovementData, &RotationData)>, //used to do input direction transform from view
) {
    let mut acceleration = Vec3::default();
    if keyboard_input.pressed(KeyCode::W) {
         acceleration += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
         acceleration += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
         acceleration += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
         acceleration += Vec3::new(-1.0, 0.0, 0.0);
    }
    // if keyboard_input.pressed(KeyCode::Space) {
    //      acceleration += Vec3::new(0.0, 1.0, 0.0);
    // }
    // if keyboard_input.pressed(KeyCode::LShift) {
    //      acceleration += Vec3::new(0.0, -1.0, 0.0);
    // }

    for (_player, mut movement_data, rotation) in player_query.iter_mut() {
        for (_player, mut camera_movement, camera_rotation) in camera_query.iter_mut() {
            let sin = camera_rotation.yaw.sin();
            let cos = camera_rotation.yaw.cos();
    
            let acceleration = Vec3::new(
                acceleration.x() * cos - acceleration.z() * sin,
                acceleration.y(),
                acceleration.z() * cos + acceleration.x() * sin,
            );
    
            movement_data.acceleration += acceleration;
        }
    }
}