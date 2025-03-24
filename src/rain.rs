use bevy::{
  app::{App, Plugin, Startup},
  color::Color,
  ecs::{bundle::Bundle, component::Component, system::Commands},
  math::primitives::Circle,
  transform::components::Transform,
};

use crate::{
  gravity::GravityComponent,
  movable::MoveComponent,
  screen_object::{ScreenObjectBundle, SpawnScreenObjectExt},
};

#[derive(Component)]
#[require(MoveComponent, GravityComponent)]
struct Rain;

#[derive(Bundle)]
struct RainBundle {
  screen_object: ScreenObjectBundle,
  rain: Rain,
}

impl RainBundle {
  const RADIUS: f32 = 10.0;

  fn spawn_rain(mut commands: Commands) {
    commands.spawn_screen_object(
      Circle::new(Self::RADIUS),
      Color::srgb(0.2, 0.6, 0.95),
      Transform::from_xyz(0.0, 0.0, -1.0),
      |screen_object| Self { screen_object, rain: Rain },
    );
  }
}

pub struct RainPlugin;

impl RainPlugin {}

impl Plugin for RainPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, RainBundle::spawn_rain);
  }
}
