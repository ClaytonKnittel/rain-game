use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin, Startup, Update},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res},
    world::World,
  },
  hierarchy::{BuildChildren, ChildBuild, Parent},
  math::{primitives::Rectangle, FloatPow, Vec2},
  time::{Time, Timer, TimerMode},
};
use rand::Rng;

use crate::{
  movable::MoveComponent,
  position::{Position, PositionPlugin},
  rain::Rain,
  screen_object::ScreenObjectBundle,
};

enum NpcState {
  Idle { timer: Timer },
  Walking { to_left: bool, timer: Timer },
  Running { to_left: bool, timer: Timer },
}

impl NpcState {
  const RUN_SPEED: f32 = 40.;
  const WALK_SPEED: f32 = 20.;

  const IDLE_DURATION: Duration = Duration::from_secs(1);
  const WALK_DURATION: Duration = Duration::from_secs(2);
  const RUN_DURATION: Duration = Duration::from_millis(1500);

  fn make_idle_state() -> Self {
    Self::Idle {
      timer: Timer::new(Self::IDLE_DURATION, TimerMode::Once),
    }
  }

  fn make_walk_state(to_left: bool) -> Self {
    Self::Walking {
      to_left,
      timer: Timer::new(Self::WALK_DURATION, TimerMode::Once),
    }
  }

  fn make_run_state(to_left: bool) -> Self {
    Self::Running {
      to_left,
      timer: Timer::new(Self::RUN_DURATION, TimerMode::Once),
    }
  }

  fn is_alert(&self) -> bool {
    matches!(self, Self::Idle { .. } | Self::Walking { .. })
  }

  fn tick(&mut self, time: &Time, npc_pos: Vec2, nearest_rain: Option<Vec2>) {
    if self.tick_timer(time).just_finished() || (self.is_alert() && nearest_rain.is_some()) {
      self.transition_states(npc_pos, nearest_rain);
    }
  }

  fn tick_timer(&mut self, time: &Time) -> &Timer {
    match self {
      Self::Idle { timer } | Self::Walking { timer, .. } | Self::Running { timer, .. } => {
        timer.tick(time.delta())
      }
    }
  }

  fn speed(&self) -> Vec2 {
    let dir = |to_left: bool| if to_left { -1. } else { 1. };
    match self {
      Self::Idle { .. } => Vec2::ZERO,
      Self::Walking { to_left, .. } => dir(*to_left) * Self::WALK_SPEED * Vec2::X,
      Self::Running { to_left, .. } => dir(*to_left) * Self::RUN_SPEED * Vec2::X,
    }
  }

  fn transition_states(&mut self, npc_pos: Vec2, nearest_rain: Option<Vec2>) {
    if let Some(nearest_rain) = nearest_rain {
      *self = Self::make_run_state(nearest_rain.x > npc_pos.x);
    } else if !matches!(self, Self::Idle { .. }) {
      *self = Self::make_idle_state();
    } else {
      *self = Self::make_walk_state(rand::rng().random_bool(0.5));
    }
  }
}

impl Default for NpcState {
  fn default() -> Self {
    Self::make_idle_state()
  }
}

#[derive(Component, Default)]
#[require(MoveComponent)]
struct Npc {
  state: NpcState,
}

#[derive(Bundle)]
struct NpcBundle {
  npc: Npc,
  pos: Position,
}

#[derive(Component)]
#[require(Position)]
struct NpcBody;

#[derive(Bundle)]
struct NpcBodyBundle {
  screen_object: ScreenObjectBundle,
  npc_body: NpcBody,
}

impl NpcBundle {
  const WIDTH: f32 = 50.;
  const HEIGHT: f32 = 80.;

  fn spawn(mut commands: Commands) {
    commands.queue(|world: &mut World| {
      let body = NpcBodyBundle {
        screen_object: ScreenObjectBundle::new(
          Rectangle::from_size(Vec2 { x: Self::WIDTH, y: Self::HEIGHT }),
          Color::srgb(0.8, 0.7, 0.6),
          // This is not getting picked up????
          2.0,
          world,
        ),
        npc_body: NpcBody,
      };

      world
        .spawn(Self {
          npc: Npc::default(),
          pos: Position(Vec2::new(0., -200.)),
        })
        .with_children(move |parent| {
          parent.spawn(body);
        });
    });
  }
}

pub struct NpcPlugin;

impl NpcPlugin {
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
    time: Res<Time>,
    mut npc_query: Query<(&mut Npc, &Position, &mut MoveComponent)>,
    rain_query: Query<&Position, With<Rain>>,
  ) {
    for (mut npc, &Position(npc_pos), mut npc_vel) in &mut npc_query {
      npc.state.tick(
        &time,
        npc_pos,
        Self::find_nearest_visible_rain_pos(npc_pos, rain_query.iter().map(|rain_pos| rain_pos.0)),
      );

      npc_vel.delta = npc.state.speed();
    }
  }

  fn sync_body_positions(
    npc_query: Query<&Position, With<Npc>>,
    mut child_query: Query<(&mut Position, &Parent), (With<NpcBody>, Without<Npc>)>,
  ) {
    for (mut child_pos, parent) in &mut child_query {
      let npc_pos = npc_query.get(parent.get()).unwrap();
      //   println!("Updating child pos {} to {}", child_pos.0, npc_pos.0);
      child_pos.0 = npc_pos.0;
    }
  }
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, Self::spawn_npcs)
      .add_systems(FixedUpdate, Self::control_npcs)
      .add_systems(
        Update,
        Self::sync_body_positions.before(PositionPlugin::sync_render_positions),
      );
  }
}
