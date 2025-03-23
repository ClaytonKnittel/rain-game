mod framerate;

use bevy::{app::App, DefaultPlugins};
use framerate::FrameratePlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameratePlugin)
    .run();
}
