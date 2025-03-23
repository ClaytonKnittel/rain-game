use bevy::{
  app::{AppExit, Plugin, Startup, Update},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, Res, ResMut},
  },
  input::{keyboard::KeyCode, ButtonInput},
  window::WindowResized,
};

use crate::win_info::WinInfo;

pub struct WorldInitPlugin;

impl WorldInitPlugin {
  fn world_init(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<WinInfo>();
  }

  fn app_exit_listener(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit: EventWriter<AppExit>,
  ) {
    if keyboard_input.pressed(KeyCode::Escape) {
      app_exit.send(AppExit::Success);
    }
  }

  fn resize_listener(mut resize_events: EventReader<WindowResized>, mut win_info: ResMut<WinInfo>) {
    for e in resize_events.read() {
      win_info.width = e.width;
      win_info.height = e.height;
      println!("width = {} height = {}", e.width, e.height);
    }
  }
}

impl Plugin for WorldInitPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app
      .add_systems(Startup, Self::world_init)
      .add_systems(Update, (Self::app_exit_listener, Self::resize_listener));
  }
}
