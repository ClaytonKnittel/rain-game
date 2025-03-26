use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query, Res, ResMut, Resource},
    world::World,
  },
  math::{primitives::Circle, Vec2},
  time::{Time, Timer, TimerMode},
};
use rand::Rng;

use crate::{
  gravity::GravityComponent, movable::MoveComponent, position::Position,
  screen_object::ScreenObjectBundle, win_info::WinInfo,
};

#[derive(Component)]
#[require(MoveComponent, GravityComponent)]
pub struct Rain;

#[derive(Bundle)]
pub struct RainBundle {
  screen_object: ScreenObjectBundle,
  pos: Position,
  rain: Rain,
}

impl RainBundle {
  pub const RADIUS: f32 = 10.0;

  fn spawn_rain(mut commands: Commands, pos: Vec2) {
    commands.queue(move |world: &mut World| {
      let screen_object = ScreenObjectBundle::new(
        Circle::new(Self::RADIUS),
        Color::srgb(0.2, 0.6, 0.95),
        -1.,
        world,
      );
      world.spawn(Self {
        screen_object,
        pos: Position(pos),
        rain: Rain,
      });
    });
  }
}

#[derive(Resource)]
struct RainTimer(Timer);

pub struct RainPlugin;

impl RainPlugin {
  // const TIMEOUT: Duration = Duration::from_secs(1);
  const TIMEOUT: Duration = Duration::from_millis(200);

  fn spawn_raindrops(
    commands: Commands,
    time: Res<Time>,
    win_info: Res<WinInfo>,
    mut timer: ResMut<RainTimer>,
  ) {
    if timer.0.tick(time.delta()).just_finished() {
      let x = rand::rng().random_range((-win_info.width / 2.)..win_info.width / 2.);
      RainBundle::spawn_rain(commands, Vec2 { x, y: win_info.height / 2. });
    }
  }

  fn despawn_raindrops(
    mut commands: Commands,
    win_info: Res<WinInfo>,
    query: Query<(Entity, &Position), With<Rain>>,
  ) {
    let min_y = -win_info.height / 2. - RainBundle::RADIUS;
    let x_bound = win_info.width / 2. + RainBundle::RADIUS;
    for (entity, Position(pos)) in &query {
      if pos.y < min_y || !(-x_bound..x_bound).contains(&pos.x) {
        commands.entity(entity).despawn();
      }
    }
  }
}

impl Plugin for RainPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(RainTimer(Timer::new(Self::TIMEOUT, TimerMode::Repeating)))
      .add_systems(
        FixedUpdate,
        (RainPlugin::spawn_raindrops, RainPlugin::despawn_raindrops),
      );
  }
}
