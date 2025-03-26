use std::collections::{hash_map::Entry, HashMap};

use bevy::{
  app::{App, Plugin, PostUpdate, Startup},
  asset::{Assets, Handle},
  color::{Color, ColorToPacked},
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::{Added, Changed},
    removal_detection::RemovedComponents,
    schedule::IntoSystemConfigs,
    system::{Commands, In, Query, ResMut, Resource, RunSystemOnce},
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
struct ColorComponent {
  color: StrictColor,
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
      color: ColorComponent { color },
      material: MeshMaterial2d(color_handle),
    }
  }
}

#[derive(Default, Resource)]
struct ColorManager {
  color_map: HashMap<StrictColor, (u32, Handle<ColorMaterial>)>,
  entity_map: HashMap<Entity, StrictColor>,
}

impl ColorManager {
  fn add_color(&mut self, entity: Entity, color: StrictColor) {
    let inserted = self.entity_map.insert(entity, color).is_none();
    debug_assert!(inserted);
  }

  fn change_color(
    &mut self,
    entity: Entity,
    new_color: StrictColor,
    materials: &mut Assets<ColorMaterial>,
  ) {
    let color_slot = self.entity_map.get_mut(&entity).unwrap();
    let old_color = *color_slot;
    if old_color == new_color {
      return;
    }

    *color_slot = new_color;

    self.untrack_color(old_color, materials);
    self.track_color(new_color, materials);
  }

  fn remove_color(&mut self, entity: Entity, materials: &mut Assets<ColorMaterial>) {
    let color = self.entity_map.remove(&entity).unwrap();
    self.untrack_color(color, materials);
  }

  fn track_color(
    &mut self,
    color: StrictColor,
    materials: &mut Assets<ColorMaterial>,
  ) -> Handle<ColorMaterial> {
    let entry = self
      .color_map
      .entry(color)
      .or_insert_with(|| (0, materials.add(Color::from(color))));

    entry.0 += 1;
    entry.1.clone_weak()
  }

  fn untrack_color(&mut self, color: StrictColor, materials: &mut Assets<ColorMaterial>) {
    match self.color_map.entry(color) {
      Entry::Occupied(mut entry) => {
        entry.get_mut().0 -= 1;
        if entry.get().0 == 0 {
          materials.remove(entry.get().1.id());
          entry.remove();
        }
      }
      _ => unreachable!(),
    }
  }
}

pub struct ColorPlugin;

impl ColorPlugin {
  fn initialize(mut commands: Commands) {
    commands.insert_resource(ColorManager::default());
  }

  fn new_color(
    In(color): In<StrictColor>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mgr: ResMut<ColorManager>,
  ) -> Handle<ColorMaterial> {
    mgr.track_color(color, &mut materials)
  }

  fn handle_new_entities(
    mut mgr: ResMut<ColorManager>,
    query: Query<(Entity, &ColorComponent), Added<ColorComponent>>,
  ) {
    for (entity, color) in &query {
      mgr.add_color(entity, color.color);
    }
  }

  fn handle_changed_entities(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mgr: ResMut<ColorManager>,
    query: Query<(Entity, &ColorComponent), Changed<ColorComponent>>,
  ) {
    for (entity, color) in &query {
      mgr.change_color(entity, color.color, &mut materials);
    }
  }

  fn handle_deleted_entities(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mgr: ResMut<ColorManager>,
    mut removed: RemovedComponents<ColorComponent>,
  ) {
    for entity in removed.read() {
      mgr.remove_color(entity, &mut materials);
    }
  }
}

impl Plugin for ColorPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, Self::initialize).add_systems(
      PostUpdate,
      (
        Self::handle_new_entities,
        Self::handle_changed_entities,
        Self::handle_deleted_entities,
      )
        .chain(),
    );
  }
}
