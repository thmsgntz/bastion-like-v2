pub mod animations;
pub mod camera;
mod input;

use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        /*window*/
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (1200., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        /* FPS debug */
        .add_plugin(LogDiagnosticsPlugin {
            debug: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        // Uncomment this to add an entity count diagnostics:
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add system info diagnostics:
        .add_plugin(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
        /* Inputs */
        .add_plugin(input::InputPlugin)
        /* Camera */
        .add_plugin(camera::CameraPlugin)
        /* Animations */
        .add_plugin(animations::AnimationsPlugin)
        .add_startup_system(setup)
        .run();
}
