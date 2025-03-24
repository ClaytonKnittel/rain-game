use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, Single},
  },
  input::{keyboard::KeyCode, ButtonInput},
  math::{primitives::Circle, FloatPow, Vec3Swizzles},
  transform::components::Transform,
};

use crate::{
  movable::{MoveComponent, MovePlugin},
  rain::{Rain, RainBundle},
  screen_object::{ScreenObjectBundle, SpawnScreenObjectExt},
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
    commands.spawn_screen_object(
      Circle::new(Self::RADIUS),
      Color::hsl(0.0, 0.95, 0.7),
      Transform::from_xyz(0.0, 0.0, 0.0),
      |screen_object| Self { screen_object, player: Player },
    );
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
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    rain_query: Query<(Entity, &Transform), With<Rain>>,
  ) {
    let player_pos = player.translation.xy();
    for (rain_entity, rain_trans) in &rain_query {
      let dist2 = (rain_trans.translation.xy() - player_pos).length_squared();
      if dist2 < (PlayerBundle::RADIUS + RainBundle::RADIUS).squared() {
        commands.entity(rain_entity).despawn();
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
      .add_systems(FixedUpdate, Self::handle_rain_collisions);
  }
}
