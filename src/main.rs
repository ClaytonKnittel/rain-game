#![allow(clippy::type_complexity)]

mod background;
mod framerate;
mod gravity;
mod movable;
mod npc;
mod player;
mod rain;
mod score;
mod shack;

use background::BackgroundPlugin;
use bevy::{
  app::App,
  asset::{AssetMetaCheck, AssetPlugin},
  prelude::PluginGroup,
  utils::default,
  window::{PresentMode, Window, WindowPlugin},
  DefaultPlugins,
};
use bevy_world_space::WorldSpacePlugins;
use framerate::FrameratePlugin;
use gravity::GravityPlugin;
use movable::MovePlugin;
use npc::NpcPlugin;
use player::PlayerPlugin;
use rain::RainPlugin;
use score::ScorePlugin;
use shack::ShackPlugin;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Rain Game".into(),
            name: Some("rain_game.app".into()),
            present_mode: PresentMode::AutoVsync,
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
    .add_plugins(WorldSpacePlugins)
    .add_plugins(FrameratePlugin)
    .add_plugins((
      PlayerPlugin,
      RainPlugin,
      NpcPlugin,
      ShackPlugin,
      BackgroundPlugin,
    ))
    .add_plugins((MovePlugin, GravityPlugin))
    .add_plugins(ScorePlugin)
    .run();
}
