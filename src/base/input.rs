use bevy::{
    prelude::*,
    input::mouse::*,
};

#[derive(Default)]
pub struct InputState {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
}
