use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::With,
    system::{Commands, Query, Res},
  },
  input::{keyboard::KeyCode, ButtonInput},
  math::primitives::Circle,
  transform::components::Transform,
};

use crate::{
  screen_object::{ScreenObjectBundle, SpawnScreenObjectExt},
  win_info::WinInfo,
};

/// Component that identifies the player.
#[derive(Component)]
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
  fn move_player(
    win_info: Res<WinInfo>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
  ) {
    for mut transform in &mut query {
      if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.y += 10.0;
      }
      if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= 10.0;
      }
      if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.y -= 10.0;
      }
      if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += 10.0;
      }
      Self::snap_in_bounds(&win_info, &mut transform);
    }
  }

  fn snap_in_bounds(win_info: &WinInfo, transform: &mut Transform) {
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

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, PlayerBundle::spawn_player)
      .add_systems(FixedUpdate, Self::move_player);
  }
}
