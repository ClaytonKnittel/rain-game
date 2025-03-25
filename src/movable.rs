use bevy::{
  app::{App, FixedUpdate, Plugin},
  ecs::{
    component::Component,
    system::{Query, Res},
  },
  math::Vec2,
  time::Time,
};

use crate::position::Position;

#[derive(Component, Default)]
pub struct MoveComponent {
  pub delta: Vec2,
}

pub struct MovePlugin;

impl MovePlugin {
  pub fn apply_moves(time: Res<Time>, mut query: Query<(&mut Position, &MoveComponent)>) {
    let dt = time.delta_secs();
    for (mut pos, MoveComponent { delta }) in &mut query {
      pos.0 += delta * dt;
    }
  }
}

impl Plugin for MovePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, MovePlugin::apply_moves);
  }
}
