use bevy::{prelude::*, render::camera::ScalingMode};

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_light);
    }
}

pub fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn setup_light(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
