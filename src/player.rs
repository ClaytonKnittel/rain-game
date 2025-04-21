use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  asset::AssetServer,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, Single},
    world::World,
  },
  image::Image,
  input::{keyboard::KeyCode, ButtonInput},
  math::{FloatPow, Vec2, Vec3},
  sprite::Sprite,
  transform::components::Transform,
};

use crate::{
  movable::{MoveComponent, MovePlugin},
  position::{OldPosition, Position},
  rain::{Rain, RainBundle},
  win_info::WinInfo,
};

/// Component that identifies the player.
#[derive(Component)]
#[require(MoveComponent)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
  sprite: Sprite,
  transform: Transform,
  pos: OldPosition,
  player: Player,
}

impl PlayerBundle {
  const IMG_WIDTH: f32 = 600.;
  // const IMG_HEIGHT: f32 = 672.;
  // const ASPECT_RATIO: f32 = Self::IMG_HEIGHT / Self::IMG_WIDTH;

  const WIDTH: f32 = 180.0;

  const Z_IDX: f32 = 1.;

  fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load::<Image>("umbrella/umbrella.png");

    let sprite = Sprite::from_image(texture);

    commands.queue(|world: &mut World| {
      world.spawn(Self {
        sprite,
        transform: Transform::from_scale(Vec3::splat(Self::WIDTH / Self::IMG_WIDTH))
          .with_translation(Self::Z_IDX * Vec3::Z),
        pos: OldPosition(Vec2::ZERO),
        player: Player,
      });
    });
  }
}

pub struct PlayerPlugin;

impl PlayerPlugin {
  const SPEED: f32 = 400.0;
  const RAIN_RESTITUTION: f32 = 0.15;

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

  fn snap_in_bounds(win_info: Res<WinInfo>, mut query: Query<&mut OldPosition, With<Player>>) {
    for mut position in &mut query {
      position.0.x = position
        .0
        .x
        .min(win_info.width / 2. - PlayerBundle::WIDTH / 2.)
        .max(-(win_info.width / 2. - PlayerBundle::WIDTH / 2.));
      position.0.y = position
        .0
        .y
        .min(win_info.height / 2. - PlayerBundle::WIDTH / 2.)
        .max(-(win_info.height / 2. - PlayerBundle::WIDTH / 2.));
    }
  }

  fn handle_rain_collisions(
    win_info: Res<WinInfo>,
    player: Single<(&OldPosition, &MoveComponent), With<Player>>,
    mut rain_query: Query<(&Position, &mut MoveComponent), (With<Rain>, Without<Player>)>,
  ) {
    let (OldPosition(player_pos), player_vel) = player.into_inner();
    for (rain_pos, mut rain_vel) in &mut rain_query {
      let diff = rain_pos.pos.to_absolute(&win_info) - player_pos;
      let dist2 = diff.length_squared();
      if diff.y >= 0.
        && dist2 < (PlayerBundle::WIDTH / 2. + RainBundle::RADIUS.to_x(&win_info)).squared()
      {
        let relative_vel = rain_vel.delta - player_vel.delta;

        let diff = diff.normalize();
        let dot = diff.dot(relative_vel);
        if dot < 0. {
          let orthogonal_vel = diff * dot;
          let impulse = -(1. + Self::RAIN_RESTITUTION) * orthogonal_vel;
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
      .add_systems(
        FixedUpdate,
        Self::snap_in_bounds.after(MovePlugin::apply_moves),
      )
      .add_systems(
        FixedUpdate,
        Self::handle_rain_collisions.before(MovePlugin::apply_moves),
      );
  }
}
