pub mod handler;

use bevy::prelude::*;
use handler::*;

pub(crate) struct AnimationsPlugin;
impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_animations)
            .add_event::<EventUpdateAnimation>()
            .add_system(setup_scene_once_loaded)
            .add_system(switch_animation);
    }
}
