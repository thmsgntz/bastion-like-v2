use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::HashMap;

pub static GLTF_PATH_EKKO_FULL_BODY: &str = "models/ekko/gltf/scene.gltf";
pub static GLTF_EKKO_NAME: &str = "Ekko_model";

pub static GLTF_SKELLY_FULL_BODY: &str = "models/skelly/scene.gltf";
pub static GLTF_SKELLY_NAME: &str = "Skelly";

pub struct EventUpdateAnimation {
    pub name: String,
    pub index_animation: usize,
}

#[derive(Resource)]
pub struct Animations(HashMap<String, Vec<Handle<AnimationClip>>>);

pub(crate) fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut animations = Animations(HashMap::new());

    /* Load Ekko */
    animations.0.insert(
        String::from(GLTF_EKKO_NAME),
        vec![
            asset_server.load(format!("{}#Animation0", GLTF_PATH_EKKO_FULL_BODY)),
            asset_server.load(format!("{}#Animation1", GLTF_PATH_EKKO_FULL_BODY)),
            asset_server.load(format!("{}#Animation2", GLTF_PATH_EKKO_FULL_BODY)),
        ],
    );
    commands.spawn(SceneBundle {
        scene: asset_server.load(format!("{}#Scene0", GLTF_PATH_EKKO_FULL_BODY)),
        ..default()
    });

    /* Load Skelly */
    animations.0.insert(
        String::from(GLTF_SKELLY_NAME),
        vec![
            asset_server.load(format!("{}#Animation0", GLTF_SKELLY_FULL_BODY)),
            asset_server.load(format!("{}#Animation1", GLTF_SKELLY_FULL_BODY)),
            asset_server.load(format!("{}#Animation2", GLTF_SKELLY_FULL_BODY)),
        ],
    );
    commands.spawn(SceneBundle {
        scene: asset_server.load(format!("{}#Scene0", GLTF_SKELLY_FULL_BODY)),
        ..default()
    });

    // Insert a resource with the current scene information
    commands.insert_resource(animations);
    // Fox
}

// Once the scene is loaded, start the animation
// pub(crate) fn setup_scene_once_loaded_old(
//     animations: Res<Animations>,
//     mut player: Query<&mut AnimationPlayer>,
//     mut done: Local<bool>,
// ) {
//     if !*done {
//         if let Ok(mut player) = player.get_single_mut() {
//             player.play(animations.0[0].clone_weak()).repeat();
//             *done = true;
//         }
//     }
// }

pub(crate) fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer, &Name), Added<AnimationPlayer>>,
) {
    for (mut player, name) in &mut players {
        let animations_for_this_player = animations.0.get(&String::from(name));
        if let Some(animations) = animations_for_this_player {
            player.play(animations[0].clone_weak()).repeat();
        }
    }
}

pub(crate) fn switch_animation(
    mut event_reader: EventReader<EventUpdateAnimation>,
    animations: Res<Animations>,
    mut q_child: Query<(&mut AnimationPlayer, &Name), With<AnimationPlayer>>,
) {
    // println!("time: {:#} {}", time.elapsed_seconds(), (time.elapsed_seconds() % 2.0));
    for event in event_reader.iter() {
        let name_entity = &event.name;
        let animation_to_play = animations
            .0
            .get(name_entity)
            .unwrap_or_else(|| panic!("No animation in Res<Animations> for {name_entity}"));

        for (mut animation_player, name) in q_child.iter_mut() {
            if name.as_str() == name_entity {
                println!("Funiing {:#}", name);
                animation_player.play_with_transition(
                    animation_to_play[event.index_animation].clone_weak(),
                    Duration::new(0, 1),
                );
            }
        }
    }
}
