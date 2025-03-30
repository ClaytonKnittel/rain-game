use bevy::{
  app::{App, FixedUpdate, Plugin},
  ecs::{
    component::Component,
    query::With,
    schedule::IntoSystemConfigs,
    system::{Query, Res},
  },
  math::Vec2,
  time::Time,
};

use crate::movable::{MoveComponent, MovePlugin};

#[derive(Component, Default)]
pub struct GravityComponent;

pub struct GravityPlugin;

impl GravityPlugin {
  const G: f32 = 400.0;

  fn apply_gravity(time: Res<Time>, mut query: Query<&mut MoveComponent, With<GravityComponent>>) {
    let g = Self::G * time.delta_secs();
    for mut move_component in &mut query {
      move_component.delta += -g * Vec2::Y;
    }
  }
}

impl Plugin for GravityPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      FixedUpdate,
      Self::apply_gravity.before(MovePlugin::apply_moves),
    );
  }
}
