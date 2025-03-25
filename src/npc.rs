use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::With,
    system::{Commands, Query, Res, Resource},
    world::World,
  },
  math::{primitives::Rectangle, FloatPow, Vec2, Vec3Swizzles},
  transform::components::Transform,
};
use rand::distr::{Bernoulli, Distribution};

use crate::{movable::MoveComponent, rain::Rain, screen_object::ScreenObjectBundle};

#[derive(Component)]
#[require(MoveComponent)]
struct Npc;

#[derive(Bundle)]
struct NpcBundle {
  screen_object: ScreenObjectBundle,
  npc: Npc,
}

impl NpcBundle {
  const WIDTH: f32 = 50.;
  const HEIGHT: f32 = 80.;

  fn spawn(mut commands: Commands) {
    commands.queue(|world: &mut World| {
      let screen_object = ScreenObjectBundle::new(
        Rectangle::from_size(Vec2 { x: Self::WIDTH, y: Self::HEIGHT }),
        Color::srgb(0.8, 0.7, 0.6),
        Transform::from_xyz(0., -200., -1.0),
        world,
      );
      world.spawn(Self { screen_object, npc: Npc });
    });
  }
}

#[derive(Resource)]
struct NpcBernoulli(Bernoulli);

impl Default for NpcBernoulli {
  fn default() -> Self {
    Self(Bernoulli::new(0.5).unwrap())
  }
}

pub struct NpcPlugin;

impl NpcPlugin {
  const RUN_SPEED: f32 = 40.;
  const WALK_SPEED: f32 = 20.;
  const SIGHT_DIST: f32 = 200.;

  fn spawn_npcs(commands: Commands) {
    NpcBundle::spawn(commands);
  }

  fn find_nearest_visible_rain_pos(
    npc_pos: Vec2,
    rain: impl IntoIterator<Item = Vec2>,
  ) -> Option<Vec2> {
    rain.into_iter().fold(None, |nearest_rain, rain_pos| {
      let dist = (rain_pos - npc_pos).length_squared();
      if dist < Self::SIGHT_DIST.squared() {
        Some(
          nearest_rain
            .map(|nearest_rain_pos| {
              if dist < (nearest_rain_pos - npc_pos).length_squared() {
                rain_pos
              } else {
                nearest_rain_pos
              }
            })
            .unwrap_or(rain_pos),
        )
      } else {
        nearest_rain
      }
    })
  }

  fn control_npcs(
    bernoulli: Res<NpcBernoulli>,
    mut npc_query: Query<(&Transform, &mut MoveComponent), With<Npc>>,
    rain_query: Query<&Transform, With<Rain>>,
  ) {
    for (npc_pos, mut npc_vel) in &mut npc_query {
      let npc_pos = npc_pos.translation.xy();

      if let Some(nearest_rain) = Self::find_nearest_visible_rain_pos(
        npc_pos,
        rain_query.iter().map(|rain_pos| rain_pos.translation.xy()),
      ) {
        npc_vel.delta = if nearest_rain.x < npc_pos.x {
          Self::RUN_SPEED * Vec2::X
        } else {
          -Self::RUN_SPEED * Vec2::X
        };
      } else {
        npc_vel.delta = if bernoulli.0.sample(&mut rand::rng()) {
          Self::WALK_SPEED * Vec2::X
        } else {
          -Self::WALK_SPEED * Vec2::X
        };
      }
    }
  }
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(NpcBernoulli::default())
      .add_systems(Startup, Self::spawn_npcs)
      .add_systems(FixedUpdate, Self::control_npcs);
  }
}
