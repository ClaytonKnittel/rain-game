use bevy::{
  app::{FixedUpdate, Plugin},
  ecs::{
    component::Component,
    system::{Query, Res},
  },
  math::Vec2,
  time::Time,
  transform::components::Transform,
};

#[derive(Component, Default)]
pub struct MoveComponent {
  pub delta: Vec2,
}

pub struct MovePlugin;

impl MovePlugin {
  pub fn apply_moves(time: Res<Time>, mut query: Query<(&mut Transform, &MoveComponent)>) {
    let dt = time.delta_secs();
    for (mut pos, MoveComponent { delta }) in &mut query {
      pos.translation.x += delta.x * dt;
      pos.translation.y += delta.y * dt;
    }
  }
}

impl Plugin for MovePlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(FixedUpdate, MovePlugin::apply_moves);
  }
}
