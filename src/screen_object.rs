use bevy::{
  asset::Assets,
  ecs::{bundle::Bundle, world::World},
  render::mesh::{Mesh, Mesh2d},
  sprite::{ColorMaterial, MeshMaterial2d},
  transform::components::Transform,
};

#[derive(Bundle)]
pub struct ScreenObjectBundle {
  pub mesh: Mesh2d,
  pub material: MeshMaterial2d<ColorMaterial>,
  pub transform: Transform,
}

impl ScreenObjectBundle {
  pub fn new(
    mesh: impl Into<Mesh>,
    mesh_material: impl Into<ColorMaterial>,
    z_idx: f32,
    world: &mut World,
  ) -> Self {
    let mesh = mesh.into();
    let mesh_material = mesh_material.into();
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = Mesh2d(meshes.add(mesh));

    let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
    let material = MeshMaterial2d(materials.add(mesh_material));

    Self {
      mesh,
      material,
      transform: Transform::from_xyz(0., 0., z_idx),
    }
  }
}
