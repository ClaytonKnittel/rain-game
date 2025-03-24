mod framerate;
mod player;
mod screen_object;
mod win_info;
mod world_init;

use bevy::{app::App, DefaultPlugins};
use framerate::FrameratePlugin;
use player::PlayerPlugin;
use world_init::WorldInitPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameratePlugin)
    .add_plugins(WorldInitPlugin)
    .add_plugins(PlayerPlugin)
    .run();
}
