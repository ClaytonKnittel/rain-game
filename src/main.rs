#![allow(clippy::type_complexity)]

mod background;
mod framerate;
mod gravity;
mod movable;
mod npc;
mod player;
mod position;
mod rain;
mod shack;
mod win_info;
mod world_init;

use background::BackgroundPlugin;
use bevy::{app::App, DefaultPlugins};
use framerate::FrameratePlugin;
use gravity::GravityPlugin;
use movable::MovePlugin;
use npc::NpcPlugin;
use player::PlayerPlugin;
use position::PositionPlugin;
use rain::RainPlugin;
use shack::ShackPlugin;
use world_init::WorldInitPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameratePlugin)
    .add_plugins(WorldInitPlugin)
    .add_plugins((
      PlayerPlugin,
      RainPlugin,
      NpcPlugin,
      ShackPlugin,
      BackgroundPlugin,
    ))
    .add_plugins((PositionPlugin, MovePlugin, GravityPlugin))
    .run();
}
