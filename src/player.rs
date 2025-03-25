use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, Single},
    world::World,
  },
  input::{keyboard::KeyCode, ButtonInput},
  math::{primitives::Circle, FloatPow, Vec3Swizzles},
  transform::components::Transform,
};

use crate::{
  movable::{MoveComponent, MovePlugin},
  rain::{Rain, RainBundle},
  screen_object::ScreenObjectBundle,
  win_info::WinInfo,
};

/// Component that identifies the player.
#[derive(Component)]
#[require(MoveComponent)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
  screen_object: ScreenObjectBundle,
  player: Player,
}

impl PlayerBundle {
  const RADIUS: f32 = 50.0;

  fn spawn_player(mut commands: Commands) {
    commands.queue(|world: &mut World| {
      let screen_object = ScreenObjectBundle::new(
        Circle::new(Self::RADIUS),
        Color::hsl(0.0, 0.95, 0.7),
        Transform::from_xyz(0.0, 0.0, 0.0),
        world,
      );
      world.spawn(Self { screen_object, player: Player });
    });
  }
}

pub struct PlayerPlugin;

impl PlayerPlugin {
  const SPEED: f32 = 200.0;

  fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MoveComponent, With<Player>>,
  ) {
    for mut move_component in &mut query {
      match (
        keyboard_input.pressed(KeyCode::KeyW),
        keyboard_input.pressed(KeyCode::KeyS),
      ) {
        (true, false) => move_component.delta.y = Self::SPEED,
        (false, true) => move_component.delta.y = -Self::SPEED,
        _ => move_component.delta.y = 0.0,
      }
      match (
        keyboard_input.pressed(KeyCode::KeyD),
        keyboard_input.pressed(KeyCode::KeyA),
      ) {
        (true, false) => move_component.delta.x = Self::SPEED,
        (false, true) => move_component.delta.x = -Self::SPEED,
        _ => move_component.delta.x = 0.0,
      }
    }
  }

  fn snap_in_bounds(win_info: Res<WinInfo>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
      transform.translation.x = transform
        .translation
        .x
        .min(win_info.width / 2. - PlayerBundle::RADIUS)
        .max(-(win_info.width / 2. - PlayerBundle::RADIUS));
      transform.translation.y = transform
        .translation
        .y
        .min(win_info.height / 2. - PlayerBundle::RADIUS)
        .max(-(win_info.height / 2. - PlayerBundle::RADIUS));
    }
  }

  fn handle_rain_collisions(
    player: Single<(&Transform, &MoveComponent), With<Player>>,
    mut rain_query: Query<(&Transform, &mut MoveComponent), (With<Rain>, Without<Player>)>,
  ) {
    let (player_pos, player_vel) = player.into_inner();
    let player_pos = player_pos.translation.xy();
    for (rain_trans, mut rain_vel) in &mut rain_query {
      let diff = rain_trans.translation.xy() - player_pos;
      let dist2 = diff.length_squared();
      if dist2 < (PlayerBundle::RADIUS + RainBundle::RADIUS).squared() {
        let relative_vel = rain_vel.delta - player_vel.delta;

        let diff = diff.normalize();
        let dot = diff.dot(relative_vel);
        if dot < 0. {
          let orthogonal_vel = diff * dot;
          let impulse = -2. * orthogonal_vel;
          rain_vel.delta += impulse;
        }
      }
    }
  }
}

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, PlayerBundle::spawn_player)
      .add_systems(
        FixedUpdate,
        Self::move_player.before(MovePlugin::apply_moves),
      )
      .add_systems(FixedUpdate, Self::snap_in_bounds)
      .add_systems(
        FixedUpdate,
        Self::handle_rain_collisions.before(MovePlugin::apply_moves),
      );
  }
}
