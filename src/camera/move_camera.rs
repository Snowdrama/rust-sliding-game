use bevy::prelude::*;
use crate::{
    camera::*,
    base::*,
};

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Camera, &mut MovementData, &RotationData)>,
) {
    let mut acceleration = Vec3::default();
    if keyboard_input.pressed(KeyCode::Up) {
         acceleration += Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::Left) {
         acceleration += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) {
         acceleration += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::Right) {
         acceleration += Vec3::new(-1.0, 0.0, 0.0);
    }

    for (_player, mut movement_data, rotation) in player_query.iter_mut() {
        let sin = rotation.yaw.sin();
        let cos = rotation.yaw.cos();

        let acceleration = Vec3::new(
            acceleration.x() * cos - acceleration.z() * sin,
            acceleration.y(),
            acceleration.z() * cos + acceleration.x() * sin,
        );

        movement_data.acceleration += acceleration;
    }
}