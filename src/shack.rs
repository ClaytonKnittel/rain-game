use bevy::{
  app::{FixedUpdate, Plugin, Startup},
  asset::Assets,
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, ResMut, Single},
  },
  math::{primitives::Rectangle, Vec2},
  render::mesh::{Mesh, Mesh2d},
  sprite::{ColorMaterial, MeshMaterial2d},
  transform::components::Transform,
};

use crate::{
  movable::MoveComponent, position::Position, rain::Rain, win_info::WinInfo,
  world_init::WorldInitPlugin,
};

#[derive(Component)]
#[require(Transform)]
struct Shack;

#[derive(Bundle)]
struct ShackBundle {
  body: Mesh2d,
  color: MeshMaterial2d<ColorMaterial>,
  pos: Position,
  shack: Shack,
}

pub struct ShackPlugin;

impl ShackPlugin {
  const WIDTH: f32 = 150.;
  const HEIGHT: f32 = 240.;

  fn spawn_shack(
    mut commands: Commands,
    win_info: Res<WinInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
  ) {
    commands.spawn(ShackBundle {
      body: Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(Self::WIDTH, Self::HEIGHT)))),
      color: MeshMaterial2d(materials.add(Color::srgb(0.6, 0.8, 0.3))),
      pos: Position(Vec2 {
        x: win_info.width / 2. - Self::WIDTH / 2.,
        y: -win_info.height / 4.,
      }),
      shack: Shack,
    });
  }

  fn handle_rain_collisions(
    shack: Single<&Position, With<Shack>>,
    mut rain_query: Query<(&Position, &mut MoveComponent), (With<Rain>, Without<Shack>)>,
  ) {
    let Position(shack_pos) = shack.into_inner();
    for (Position(rain_pos), mut rain_vel) in &mut rain_query {
      let diff = rain_pos - shack_pos;
      if diff.x.abs() <= Self::WIDTH / 2. && rain_vel.delta.y < 0. && diff.y <= Self::HEIGHT / 2. {
        rain_vel.delta.y = -rain_vel.delta.y * 0.3;
        rain_vel.delta.x += 1.;
      }
    }
  }
}

impl Plugin for ShackPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_systems(
        Startup,
        Self::spawn_shack.after(WorldInitPlugin::world_init),
      )
      .add_systems(FixedUpdate, Self::handle_rain_collisions);
  }
}
