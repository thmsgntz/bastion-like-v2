use bevy::prelude::*;

pub static GLTF_PATH_FULL_BODY: &str = "models/ekko/gltf/scene.gltf";

pub(crate) struct AnimationsPlugin;
impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_animations)
        .add_system(setup_scene_once_loaded);
    }
}


#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load(format!("{}#Animation0", GLTF_PATH_FULL_BODY)),
    ]));
    // Fox
    println!("Loading..");
    commands.spawn(SceneBundle {
        scene: asset_server.load(format!("{}#Scene0", GLTF_PATH_FULL_BODY)),
        ..default()
    });
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
    ) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            println!("DONE");
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}