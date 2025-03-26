use bevy::{
  app::{App, Plugin, PostUpdate},
  asset::{Assets, Handle},
  color::{Color, ColorToPacked},
  ecs::{
    bundle::Bundle,
    component::Component,
    query::Changed,
    system::{In, Query, ResMut, RunSystemOnce},
    world::World,
  },
  sprite::{ColorMaterial, MeshMaterial2d},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StrictColor {
  r: u8,
  g: u8,
  b: u8,
}

impl StrictColor {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

impl From<Color> for StrictColor {
  fn from(value: Color) -> Self {
    let [r, g, b, _] = value.to_srgba().to_u8_array();
    Self { r, g, b }
  }
}

impl From<StrictColor> for Color {
  fn from(value: StrictColor) -> Color {
    Color::srgb_u8(value.r, value.g, value.b)
  }
}

#[derive(Component)]
pub struct ColorComponent {
  color: StrictColor,
  desired_color: StrictColor,
}

impl ColorComponent {
  fn new(color: StrictColor) -> Self {
    Self { color, desired_color: color }
  }

  pub fn set_color(&mut self, desired_color: StrictColor) {
    self.desired_color = desired_color;
  }

  fn sync_color(
    &mut self,
    material: &mut MeshMaterial2d<ColorMaterial>,
    materials: &mut Assets<ColorMaterial>,
  ) {
    if self.desired_color == self.color {
      return;
    }

    material.0 = materials.add(Color::from(self.desired_color));
    self.color = self.desired_color;
  }
}

#[derive(Bundle)]
pub struct ColorBundle {
  color: ColorComponent,
  material: MeshMaterial2d<ColorMaterial>,
}

impl ColorBundle {
  pub fn new(world: &mut World, color: StrictColor) -> Self {
    let color_handle = world
      .run_system_once_with(color, ColorPlugin::new_color)
      .unwrap();

    Self {
      color: ColorComponent::new(color),
      material: MeshMaterial2d(color_handle),
    }
  }
}

pub struct ColorPlugin;

impl ColorPlugin {
  fn new_color(
    In(color): In<StrictColor>,
    mut materials: ResMut<Assets<ColorMaterial>>,
  ) -> Handle<ColorMaterial> {
    materials.add(Color::from(color))
  }

  fn handle_changed_entities(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
      (&mut ColorComponent, &mut MeshMaterial2d<ColorMaterial>),
      Changed<ColorComponent>,
    >,
  ) {
    for (mut color, mut material) in &mut query {
      color.sync_color(&mut material, &mut materials);
    }
  }
}

impl Plugin for ColorPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostUpdate, Self::handle_changed_entities);
  }
}
