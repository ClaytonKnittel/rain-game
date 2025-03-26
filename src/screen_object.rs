use bevy::{
  asset::Assets,
  color::Color,
  ecs::{bundle::Bundle, world::World},
  render::mesh::{Mesh, Mesh2d},
  transform::components::Transform,
};

use crate::color::ColorBundle;

#[derive(Bundle)]
pub struct ScreenObjectBundle {
  mesh: Mesh2d,
  transform: Transform,
  color: ColorBundle,
}

impl ScreenObjectBundle {
  pub fn new(
    mesh: impl Into<Mesh>,
    color: impl Into<Color>,
    z_idx: f32,
    world: &mut World,
  ) -> Self {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = Mesh2d(meshes.add(mesh.into()));

    Self {
      mesh,
      transform: Transform::from_xyz(0., 0., z_idx),
      color: ColorBundle::new(world, color.into().into()),
    }
  }
}
