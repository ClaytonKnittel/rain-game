use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin, Startup},
  asset::{AssetServer, Handle},
  ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, Resource, Single},
  },
  image::Image,
  math::{Vec2, Vec3},
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
};

use crate::{
  movable::MoveComponent, position::Position, rain::Rain, win_info::WinInfo,
  world_init::WorldInitPlugin,
};

#[derive(Component)]
#[require(Transform)]
struct Shack {
  timer: Timer,
  animation_idx: usize,
}

impl Shack {
  const ANIMATION_SPEED: Duration = Duration::from_millis(300);

  fn new() -> Self {
    Self {
      timer: Timer::new(Self::ANIMATION_SPEED, TimerMode::Repeating),
      animation_idx: 0,
    }
  }

  fn tick(&mut self, delta: Duration, shack_assets: &ShackAssets, sprite: &mut Sprite) {
    self.timer.tick(delta);
    if self.timer.just_finished() {
      self.animation_idx = (self.animation_idx + 1) % shack_assets.shack_sprites.len();
      sprite.image = shack_assets.shack_sprites[self.animation_idx].clone_weak();
    }
  }
}

#[derive(Bundle)]
struct ShackBundle {
  sprite: Sprite,
  transform: Transform,
  pos: Position,
  shack: Shack,
}

#[derive(Resource)]
struct ShackAssets {
  shack_sprites: [Handle<Image>; 4],
}

pub struct ShackPlugin;

impl ShackPlugin {
  const IMG_WIDTH: f32 = 1500.;

  // 3 3 5 5, 2 3 5 7
  const WIDTH: f32 = 300.; // 15, 14
  const HEIGHT: f32 = 280.;

  const RAIN_RESTITUTION: f32 = 0.3;

  fn initialize_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shack_sprites = [1, 2, 3, 2].map(|idx| asset_server.load(format!("shack/shack_{idx}.png")));
    commands.insert_resource(ShackAssets { shack_sprites });
  }

  fn spawn_shack(mut commands: Commands, win_info: Res<WinInfo>, shack_assets: Res<ShackAssets>) {
    commands.spawn(ShackBundle {
      sprite: Sprite::from_image(shack_assets.shack_sprites[0].clone_weak()),
      transform: Transform::from_scale(Vec3::splat(Self::WIDTH / Self::IMG_WIDTH)),
      pos: Position(Vec2 {
        x: win_info.width / 2. - Self::WIDTH / 2.,
        y: -win_info.height * 0.4 + Self::HEIGHT / 2.,
      }),
      shack: Shack::new(),
    });
  }

  fn tick(
    time: Res<Time>,
    shack_assets: Res<ShackAssets>,
    shack: Single<(&mut Shack, &mut Sprite)>,
  ) {
    let (mut shack, mut sprite) = shack.into_inner();
    shack.tick(time.delta(), &shack_assets, &mut sprite);
  }

  fn handle_rain_collisions(
    shack: Single<&Position, With<Shack>>,
    mut rain_query: Query<(&Position, &mut MoveComponent), (With<Rain>, Without<Shack>)>,
  ) {
    let Position(shack_pos) = shack.into_inner();
    for (Position(rain_pos), mut rain_vel) in &mut rain_query {
      let diff = rain_pos - shack_pos;

      let tl_corner = shack_pos + Vec2::new(-Self::WIDTH / 2., Self::HEIGHT / 2.);
      let from_tl_corner = rain_pos - tl_corner;

      if diff.x.abs() <= Self::WIDTH / 2. && diff.y.abs() <= Self::HEIGHT / 2. {
        if from_tl_corner.y < -from_tl_corner.x {
          if rain_vel.delta.x > 0. {
            rain_vel.delta.x = -rain_vel.delta.x * Self::RAIN_RESTITUTION;
          }
        } else if rain_vel.delta.y < 0. {
          rain_vel.delta.y = -rain_vel.delta.y * Self::RAIN_RESTITUTION;
          rain_vel.delta.x += 1.;
        }
      }
    }
  }
}

impl Plugin for ShackPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Startup,
        (
          Self::initialize_plugin,
          Self::spawn_shack.after(WorldInitPlugin::world_init),
        )
          .chain(),
      )
      .add_systems(FixedUpdate, (Self::tick, Self::handle_rain_collisions));
  }
}
