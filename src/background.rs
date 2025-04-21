use bevy::{
  app::{Plugin, Startup},
  asset::AssetServer,
  ecs::{
    bundle::Bundle,
    component::Component,
    schedule::IntoSystemConfigs,
    system::{Commands, Res},
  },
  sprite::Sprite,
};

use crate::{
  position::Position,
  world_init::WorldInitPlugin,
  world_unit::{WorldUnit, WorldVec2},
};

#[derive(Component)]
struct Background;

#[derive(Bundle)]
struct BackgroundBundle {
  sprite: Sprite,
  pos: Position,
  background: Background,
}

pub struct BackgroundPlugin;

impl BackgroundPlugin {
  const IMG_WIDTH: u32 = 1280;
  // const IMG_HEIGHT: u32 = 720;

  const Z_IDX: f32 = -10.;

  fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load("background/background.jpg"));

    commands.spawn(BackgroundBundle {
      sprite,
      pos: Position::new(
        WorldVec2::ZERO,
        WorldUnit::SCREEN_WIDTH,
        Self::IMG_WIDTH,
        Self::Z_IDX,
      ),
      background: Background,
    });
  }
}

impl Plugin for BackgroundPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(
      Startup,
      BackgroundPlugin::spawn.after(WorldInitPlugin::world_init),
    );
  }
}
