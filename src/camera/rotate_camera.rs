use bevy::{
    prelude::*,
    input::mouse::*,
};

use crate::{
    base::*,
    camera::*,
};

pub fn rotate_camera(
    mut state: Local<InputState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut player_query: Query<(&Camera, &mut RotationData)>,
) {
    let mut mouse_delta = Vec2::default();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        mouse_delta += event.delta * 0.016 * 0.1;
    }

    for (_, mut look_direction) in player_query.iter_mut() {
        look_direction.yaw += mouse_delta.x();
        // We're not going to allow pitch just yaw
        // look_direction.pitch += mouse_delta.y();

        // println!("yaw:{}, pitch:{}", look_direction.yaw * (180.0 / 3.14), look_direction.pitch * (180.0 / 3.14));
    }
}