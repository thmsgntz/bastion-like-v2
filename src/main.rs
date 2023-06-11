pub mod camera;

use bevy::prelude::*;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    camera::setup_camera(&mut commands);
    camera::setup_light(&mut commands);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cubes
    //    commands.spawn(PbrBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(1.5, 0.5, 1.5),
    //        ..default()
    //    });
    //    commands.spawn(PbrBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(1.5, 0.5, -1.5),
    //        ..default()
    //    });
    //    commands.spawn(PbrBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
    //        ..default()
    //    });
    //    commands.spawn(PbrBundle {
    //        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
    //        ..default()
    //    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
