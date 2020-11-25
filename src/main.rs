use bevy::{
    prelude::*, 
    // render::camera::Camera, 
    window::WindowMode
};

mod base;
mod player;
mod camera;
// use base::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy-Game".to_string(),
            width: 1600,
            height: 900,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(camera::move_camera.system())
        .add_system(camera::rotate_camera.system())
        .add_system(player::move_player.system())
        // .add_system(player::update_player_look_direction.system())
        // .add_system(player::update_player_mouse_input.system())
        .add_system(base::apply_movement.system())
        .add_system(base::apply_rotation.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        //player
        .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
                ..Default::default()
        })
        .with(player::Player)
        .with(base::MovementData::default())
        .with(base::RotationData::default())
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-8.0, 8.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(camera::Camera)
        .with(base::MovementData::default())
        .with(base::RotationData{
            yaw: -135.0 * (3.14 / 180.0),
            pitch: 30.0 * (3.14 / 180.0),
            roll: 0.0 * (3.14 / 180.0),
        });
}