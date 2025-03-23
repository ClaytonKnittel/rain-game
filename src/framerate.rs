use bevy::{
  app::{App, Plugin, Update},
  ecs::system::{Res, ResMut, Resource},
  time::{Time, Timer},
};

#[derive(Resource)]
struct FramerateDisplayTimer(Timer);

#[derive(Resource)]
struct FramerateCounter {
  frames: u32,
}

impl Default for FramerateCounter {
  fn default() -> Self {
    Self { frames: 0 }
  }
}

pub struct FrameratePlugin;

impl FrameratePlugin {
  fn print_framerate(
    time: Res<Time>,
    mut timer: ResMut<FramerateDisplayTimer>,
    mut framerate_counter: ResMut<FramerateCounter>,
  ) {
    framerate_counter.frames += 1;
    if timer.0.tick(time.delta()).just_finished() {
      println!("{} fps", framerate_counter.frames);
      framerate_counter.frames = 0;
    }
  }
}

impl Plugin for FrameratePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(FramerateDisplayTimer(Timer::from_seconds(
        1.,
        bevy::time::TimerMode::Repeating,
      )))
      .insert_resource(FramerateCounter::default())
      .add_systems(Update, FrameratePlugin::print_framerate);
  }
}
