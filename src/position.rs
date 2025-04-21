use bevy::{
  app::{App, Plugin, Update},
  ecs::{
    component::Component,
    system::{Query, Res},
  },
  transform::components::Transform,
};

use crate::{
  win_info::WinInfo,
  world_unit::{WorldUnit, WorldVec2},
};

#[derive(Component, Default)]
pub struct Position {
  /// The position of the center of this entity, where (0, 0) is the center of the screen.
  pub pos: WorldVec2,
  /// The scaled width of the image in terms of world units.
  pub scale: WorldUnit,
  /// The original width of the image in pixels.
  pub image_width: u32,
}

impl Position {
  pub fn new(pos: WorldVec2, scale: WorldUnit, image_width: u32) -> Self {
    Self { pos, scale, image_width }
  }
}

pub struct PositionPlugin;

impl PositionPlugin {
  fn sync_render_positions(win_info: Res<WinInfo>, mut query: Query<(&Position, &mut Transform)>) {
    for (Position { pos, scale, image_width }, mut transform) in &mut query {
      let pos = pos.to_absolute(&win_info);
      let image_width = *image_width as f32;

      transform.translation.x = pos.x;
      transform.translation.y = pos.y;
      transform.scale.x = scale.to_x(&win_info) / image_width;
      transform.scale.y = scale.to_y(&win_info) / image_width;
    }
  }
}

impl Plugin for PositionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, PositionPlugin::sync_render_positions);
  }
}
