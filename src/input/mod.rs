use bevy::prelude::*;

use crate::animations::handler::{EventUpdateAnimation, GLTF_EKKO_NAME, GLTF_SKELLY_NAME};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_inputs);
    }
}

fn keyboard_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<EventUpdateAnimation>,
) {
    let mut index_animation: usize = 0;
    let mut name: String = String::from("");

    if keyboard_input.pressed(KeyCode::Q) {
        name = String::from(GLTF_SKELLY_NAME);
        index_animation = 0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        name = String::from(GLTF_SKELLY_NAME);
        index_animation = 1;
    }

    if keyboard_input.pressed(KeyCode::D) {
        name = String::from(GLTF_SKELLY_NAME);
        index_animation = 2;
    }

    if keyboard_input.pressed(KeyCode::A) {
        name = String::from(GLTF_EKKO_NAME);
        index_animation = 0;
    }

    if keyboard_input.pressed(KeyCode::Z) {
        name = String::from(GLTF_EKKO_NAME);
        index_animation = 1;
    }

    if keyboard_input.pressed(KeyCode::E) {
        name = String::from(GLTF_EKKO_NAME);
        index_animation = 2;
    }

    if name.is_empty() {
        event_writer.send(EventUpdateAnimation {
            name,
            index_animation,
        });
    }
}
