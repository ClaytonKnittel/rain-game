#![allow(clippy::type_complexity)]

mod framerate;
mod gravity;
mod movable;
mod npc;
mod player;
mod rain;
mod screen_object;
mod win_info;
mod world_init;

use bevy::{app::App, DefaultPlugins};
use framerate::FrameratePlugin;
use gravity::GravityPlugin;
use movable::MovePlugin;
use npc::NpcPlugin;
use player::PlayerPlugin;
use rain::RainPlugin;
use world_init::WorldInitPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameratePlugin)
    .add_plugins(WorldInitPlugin)
    .add_plugins((PlayerPlugin, RainPlugin, NpcPlugin))
    .add_plugins((MovePlugin, GravityPlugin))
    .run();
}
