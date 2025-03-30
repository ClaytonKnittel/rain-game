use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct WinInfo {
  pub width: f32,
  pub height: f32,
}

impl Default for WinInfo {
  fn default() -> Self {
    Self { width: 1280., height: 720. }
  }
}
