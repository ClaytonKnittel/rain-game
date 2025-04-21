use bevy::{
  app::{App, FixedUpdate, Plugin},
  ecs::{
    component::Component,
    system::{Query, Res},
  },
  time::Time,
};

use crate::{position::Position, world_unit::WorldVec2};

#[derive(Component, Default)]
pub struct MoveComponent {
  pub delta: WorldVec2,
}

pub struct MovePlugin;

impl MovePlugin {
  pub fn apply_moves(time: Res<Time>, mut query: Query<(&mut Position, &MoveComponent)>) {
    let dt = time.delta_secs();
    for (mut pos, MoveComponent { delta }) in &mut query {
      let delta = *delta * dt;
      pos.pos += delta;
    }
  }
}

impl Plugin for MovePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, MovePlugin::apply_moves);
  }
}
