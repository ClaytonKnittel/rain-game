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
  sprite::Sprite,
};

use crate::{
  movable::{MoveComponent, MovePlugin},
  position::Position,
  rain::{Rain, RainBundle},
  world_unit::{WorldUnit, WorldVec2},
};

/// Component that identifies the player.
#[derive(Component)]
#[require(MoveComponent)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
  sprite: Sprite,
  pos: Position,
  player: Player,
}

impl PlayerBundle {
  const IMG_WIDTH: u32 = 600;
  // const IMG_HEIGHT: u32 = 672;
  // const ASPECT_RATIO: f32 = Self::IMG_HEIGHT as f32 / Self::IMG_WIDTH as f32;

  const WIDTH: WorldUnit = WorldUnit::new(7.);

  const Z_IDX: f32 = 1.;

  fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load::<Image>("umbrella/umbrella.png");

    let sprite = Sprite::from_image(texture);

    commands.queue(|world: &mut World| {
      world.spawn(Self {
        sprite,
        pos: Position::new(WorldVec2::ZERO, Self::WIDTH, Self::IMG_WIDTH, Self::Z_IDX),
        player: Player,
      });
    });
  }
}

pub struct PlayerPlugin;

impl PlayerPlugin {
  const SPEED: WorldUnit = WorldUnit::new(16.);
  const RAIN_RESTITUTION: f32 = 0.15;

  fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MoveComponent, With<Player>>,
  ) {
    for mut move_component in &mut query {
      match (
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp),
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown),
      ) {
        (true, false) => move_component.delta.y = Self::SPEED,
        (false, true) => move_component.delta.y = -Self::SPEED,
        _ => move_component.delta.y = WorldUnit::ZERO,
      }
      match (
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight),
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft),
      ) {
        (true, false) => move_component.delta.x = Self::SPEED,
        (false, true) => move_component.delta.x = -Self::SPEED,
        _ => move_component.delta.x = WorldUnit::ZERO,
      }
    }
  }

  fn snap_in_bounds(mut query: Query<&mut Position, With<Player>>) {
    for mut pos in &mut query {
      let pos = &mut pos.pos;
      pos.x = pos
        .x
        .min(WorldUnit::RIGHT - PlayerBundle::WIDTH / 2.)
        .max(WorldUnit::LEFT + PlayerBundle::WIDTH / 2.);
      pos.y = pos
        .y
        .min(WorldUnit::TOP - PlayerBundle::WIDTH / 2.)
        .max(WorldUnit::BOTTOM + PlayerBundle::WIDTH / 2.);
    }
  }

  fn handle_rain_collisions(
    player: Single<(&Position, &MoveComponent), With<Player>>,
    mut rain_query: Query<(&Position, &mut MoveComponent), (With<Rain>, Without<Player>)>,
  ) {
    let (player_pos, player_vel) = player.into_inner();
    for (rain_pos, mut rain_vel) in &mut rain_query {
      let diff = rain_pos.pos - player_pos.pos;
      let dist2 = diff.length_squared();
      if diff.y >= WorldUnit::ZERO
        && dist2 < (PlayerBundle::WIDTH / 2. + RainBundle::RADIUS).squared()
      {
        let relative_vel = rain_vel.delta - player_vel.delta;

        let diff = diff.normalized();
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
