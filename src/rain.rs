use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  asset::{AssetServer, Handle},
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query, Res, ResMut, Resource},
    world::World,
  },
  image::Image,
  math::{ops::atan2, Quat, Vec2},
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
};

use crate::{
  gravity::GravityComponent,
  movable::MoveComponent,
  position::Position,
  world_unit::{WorldUnit, WorldVec2},
};

#[derive(Component)]
#[require(MoveComponent, GravityComponent, Transform)]
pub struct Rain;

#[derive(Bundle)]
pub struct RainBundle {
  sprite: Sprite,
  pos: Position,
  rain: Rain,
}

impl RainBundle {
  // const IMG_WIDTH: f32 = 600.;
  // const IMG_HEIGHT: f32 = 672.;

  const RAIN_WIDTH: u32 = 233;
  // const RAIN_HEIGHT: f32 = 390.;

  pub const RADIUS: WorldUnit = WorldUnit::new(0.4);

  const Z_IDX: f32 = 0.;

  fn spawn_rain(mut commands: Commands, rain_image: Handle<Image>, pos: WorldVec2) {
    commands.queue(move |world: &mut World| {
      world.spawn(Self {
        sprite: Sprite::from_image(rain_image),
        pos: Position::new(pos, Self::RADIUS, Self::RAIN_WIDTH, Self::Z_IDX),
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

  fn spawn_raindrops(commands: Commands, time: Res<Time>, mut resources: ResMut<RainResources>) {
    if resources.timer.tick(time.delta()).just_finished() {
      RainBundle::spawn_rain(
        commands,
        resources.rain_image.clone_weak(),
        WorldVec2::normalized(2. * fastrand::f32() - 1., 1.),
      );
    }
  }

  fn despawn_raindrops(mut commands: Commands, query: Query<(Entity, &Position), With<Rain>>) {
    let min_y = WorldUnit::BOTTOM - RainBundle::RADIUS;
    let x_bound = WorldUnit::RIGHT + RainBundle::RADIUS;
    for (entity, Position { pos, .. }) in &query {
      if pos.y < min_y || !(-x_bound..x_bound).contains(&pos.x) {
        commands.entity(entity).despawn();
      }
    }
  }

  fn rotate_raindrops(mut query: Query<(&MoveComponent, &mut Transform), With<Rain>>) {
    for (movement, mut transform) in &mut query {
      let delta = movement.delta.try_normalize().unwrap_or(-Vec2::Y);
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
