use bevy::{
  asset::Assets,
  ecs::{bundle::Bundle, system::Commands, world::World},
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

pub trait SpawnScreenObjectExt {
  fn spawn_screen_object<F, E>(
    &mut self,
    mesh: impl Into<Mesh>,
    mesh_material: impl Into<ColorMaterial>,
    transform: Transform,
    entity_constructor: F,
  ) where
    F: FnOnce(ScreenObjectBundle) -> E + Send + 'static,
    E: Bundle;
}

impl<'w, 's> SpawnScreenObjectExt for Commands<'w, 's> {
  fn spawn_screen_object<F, E>(
    &mut self,
    mesh: impl Into<Mesh>,
    mesh_material: impl Into<ColorMaterial>,
    transform: Transform,
    entity_constructor: F,
  ) where
    F: FnOnce(ScreenObjectBundle) -> E + Send + 'static,
    E: Bundle,
  {
    let mesh = mesh.into();
    let mesh_material = mesh_material.into();
    self.queue(move |world: &mut World| {
      let mut meshes = world.resource_mut::<Assets<Mesh>>();
      let mesh = Mesh2d(meshes.add(mesh));

      let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
      let material = MeshMaterial2d(materials.add(mesh_material));

      let screen_object = ScreenObjectBundle { mesh, material, transform };
      let entity = entity_constructor(screen_object);
      world.spawn(entity);
    });
  }
}
