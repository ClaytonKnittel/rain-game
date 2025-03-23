use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  asset::Assets,
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::With,
    system::{Commands, Query, Res, ResMut},
  },
  input::{keyboard::KeyCode, ButtonInput},
  math::primitives::Circle,
  render::mesh::{Mesh, Mesh2d},
  sprite::{ColorMaterial, MeshMaterial2d},
  transform::components::Transform,
};

/// Component that identifies the player.
#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
  mesh: Mesh2d,
  material: MeshMaterial2d<ColorMaterial>,
  transform: Transform,
  player: Player,
}

impl PlayerBundle {
  fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Self {
    Self {
      mesh: Mesh2d(meshes.add(Circle::new(50.0))),
      material: MeshMaterial2d(materials.add(Color::hsl(0.0, 0.95, 0.7))),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      player: Player,
    }
  }
}

pub struct PlayerPlugin;

impl PlayerPlugin {
  fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
  ) {
    commands.spawn(PlayerBundle::new(&mut meshes, &mut materials));
  }

  fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
  ) {
    if keyboard_input.pressed(KeyCode::Space) {
      for mut transform in &mut query {
        transform.translation.x += 1.0;
      }
    }
  }
}

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, Self::spawn_player)
      .add_systems(FixedUpdate, Self::move_player);
  }
}
