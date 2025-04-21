use bevy::{
  app::{Plugin, Startup},
  asset::AssetServer,
  ecs::{
    bundle::Bundle,
    component::Component,
    schedule::IntoSystemConfigs,
    system::{Commands, Res},
  },
  math::{Vec2, Vec3},
  sprite::Sprite,
  transform::components::Transform,
};

use crate::{position::OldPosition, win_info::WinInfo, world_init::WorldInitPlugin};

#[derive(Component)]
struct Background;

#[derive(Bundle)]
struct BackgroundBundle {
  sprite: Sprite,
  transform: Transform,
  pos: OldPosition,
  background: Background,
}

pub struct BackgroundPlugin;

impl BackgroundPlugin {
  const IMG_WIDTH: f32 = 1280.;
  // const IMG_HEIGHT: f32 = 720.;

  fn spawn(mut commands: Commands, win_info: Res<WinInfo>, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load("background/background.jpg"));

    commands.spawn(BackgroundBundle {
      sprite,
      transform: Transform::from_scale(Vec3::splat(win_info.width / Self::IMG_WIDTH))
        .with_translation(-10. * Vec3::Z),
      pos: OldPosition(Vec2::ZERO),
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
