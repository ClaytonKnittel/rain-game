use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  asset::{AssetServer, Handle},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query, Res, ResMut, Resource},
    world::World,
  },
  image::Image,
  math::{ops::atan2, primitives::Circle, Quat, Vec2, Vec3},
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
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
  sprite: Sprite,
  transform: Transform,
  pos: Position,
  rain: Rain,
}

impl RainBundle {
  const IMG_WIDTH: f32 = 600.;
  const IMG_HEIGHT: f32 = 672.;

  const RAIN_WIDTH: f32 = 233.;
  const RAIN_HEIGHT: f32 = 390.;

  pub const RADIUS: f32 = 10.0;

  fn spawn_rain(mut commands: Commands, rain_image: Handle<Image>, pos: Vec2) {
    commands.queue(move |world: &mut World| {
      world.spawn(Self {
        sprite: Sprite::from_image(rain_image),
        transform: Transform::from_scale(Vec3::splat(Self::RADIUS / Self::RAIN_WIDTH)),
        pos: Position(pos),
        rain: Rain,
      });
    });
  }
}

#[derive(Resource)]
struct RainResources {
  rain_image: Handle<Image>,
  timer: Timer,
}

pub struct RainPlugin;

impl RainPlugin {
  // const TIMEOUT: Duration = Duration::from_secs(1);
  const TIMEOUT: Duration = Duration::from_millis(200);

  fn initialize_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rain_image = asset_server.load::<Image>("raindrop/raindrop.png");
    commands.insert_resource(RainResources {
      rain_image,
      timer: Timer::new(Self::TIMEOUT, TimerMode::Repeating),
    });
  }

  fn spawn_raindrops(
    commands: Commands,
    time: Res<Time>,
    win_info: Res<WinInfo>,
    mut resources: ResMut<RainResources>,
  ) {
    if resources.timer.tick(time.delta()).just_finished() {
      let x = rand::rng().random_range((-win_info.width / 2.)..win_info.width / 2.);
      RainBundle::spawn_rain(
        commands,
        resources.rain_image.clone_weak(),
        Vec2 { x, y: win_info.height / 2. },
      );
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

  fn rotate_raindrops(mut query: Query<(&MoveComponent, &mut Transform), With<Rain>>) {
    for (movement, mut transform) in &mut query {
      let delta = movement.delta.try_normalize().unwrap_or(Vec2::Y);
      let angle = atan2(delta.x, -delta.y);
      *transform = transform.with_rotation(Quat::from_rotation_z(angle));
    }
  }
}

impl Plugin for RainPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, Self::initialize_plugin)
      .add_systems(
        FixedUpdate,
        (
          Self::spawn_raindrops,
          Self::despawn_raindrops,
          Self::rotate_raindrops,
        ),
      );
  }
}
