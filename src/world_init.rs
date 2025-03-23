use bevy::{
  app::{Plugin, Startup},
  core_pipeline::core_2d::Camera2d,
  ecs::system::Commands,
};

pub struct WorldInitPlugin;

impl WorldInitPlugin {
  fn world_init(mut commands: Commands) {
    commands.spawn(Camera2d);
  }
}

impl Plugin for WorldInitPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Startup, Self::world_init);
  }
}
