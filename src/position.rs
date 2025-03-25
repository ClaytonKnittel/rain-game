use bevy::{
  app::{App, Plugin, Update},
  ecs::{component::Component, system::Query},
  math::Vec2,
  transform::components::Transform,
};

#[derive(Component, Default)]
pub struct Position(pub Vec2);

pub struct PositionPlugin;

impl PositionPlugin {
  pub fn sync_render_positions(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
      transform.translation.x = pos.0.x;
      transform.translation.y = pos.0.y;
    }
  }
}

impl Plugin for PositionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, PositionPlugin::sync_render_positions);
  }
}
