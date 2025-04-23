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
mod world_unit;

use background::BackgroundPlugin;
use bevy::{
  app::App,
  asset::{AssetMetaCheck, AssetPlugin},
  prelude::PluginGroup,
  utils::default,
  window::{PresentMode, Window, WindowPlugin},
  DefaultPlugins,
};
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
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Rain Game".into(),
            name: Some("rain_game.app".into()),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
          }),
          ..default()
        })
        .set(AssetPlugin {
          meta_check: AssetMetaCheck::Never,
          ..default()
        }),
    )
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
