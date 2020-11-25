use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct RotationData {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

pub fn apply_rotation(
    mut entities: Query<(&mut RotationData, &mut Transform)>,
) {
    for (rotation_data, mut transform) in entities.iter_mut() {
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(
            direction.x(),
            direction.y() * rotation_data.pitch.cos() - direction.z() * rotation_data.pitch.sin(),
            direction.z() * rotation_data.pitch.cos() - direction.y() * rotation_data.pitch.sin(),
        );

        let direction = Vec3::new(
            direction.x() * rotation_data.yaw.cos() - direction.z() * rotation_data.yaw.sin(),
            direction.y(),
            direction.z() * rotation_data.yaw.cos() - direction.x() * rotation_data.yaw.sin(),
        );

        let direction = direction.normalize();
        *transform = Transform::from_translation(transform.translation).looking_at(transform.translation + direction, Vec3::unit_y());
    }
}
