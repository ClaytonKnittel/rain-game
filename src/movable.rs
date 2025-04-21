use bevy::{
  app::{App, FixedUpdate, Plugin},
  ecs::{
    component::Component,
    system::{Query, Res},
  },
  math::Vec2,
  time::Time,
};

use crate::{
  position::{OldPosition, Position},
  world_unit::WorldVec2,
};

#[derive(Component, Default)]
pub struct MoveComponent {
  pub delta: Vec2,
}

pub struct MovePlugin;

impl MovePlugin {
  pub fn apply_moves(time: Res<Time>, mut query: Query<(&mut Position, &MoveComponent)>) {
    let dt = time.delta_secs();
    for (mut pos, MoveComponent { delta }) in &mut query {
      let delta = delta * dt;
      pos.pos += WorldVec2::normalized(delta.x / (1280. / 2.), delta.y / (720. / 2.));
    }
  }

  pub fn apply_old_moves(time: Res<Time>, mut query: Query<(&mut OldPosition, &MoveComponent)>) {
    let dt = time.delta_secs();
    for (mut pos, MoveComponent { delta }) in &mut query {
      pos.0 += delta * dt;
    }
  }
}

impl Plugin for MovePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(FixedUpdate, MovePlugin::apply_moves)
      .add_systems(FixedUpdate, MovePlugin::apply_old_moves);
  }
}
