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
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
};

use crate::{
  movable::MoveComponent,
  position::Position,
  rain::Rain,
  world_init::WorldInitPlugin,
  world_unit::{WorldUnit, WorldVec2},
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
  pos: Position,
  shack: Shack,
}

#[derive(Resource)]
struct ShackAssets {
  shack_sprites: [Handle<Image>; 4],
}

pub struct ShackPlugin;

impl ShackPlugin {
  const IMG_WIDTH: u32 = 1500;

  const WIDTH: WorldUnit = WorldUnit::new(11.7);
  const HEIGHT: WorldUnit = WorldUnit::new(10.9);

  const Z_IDX: f32 = 2.;

  const RAIN_RESTITUTION: f32 = 0.3;

  fn initialize_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shack_sprites = [1, 2, 3, 2].map(|idx| asset_server.load(format!("shack/shack_{idx}.png")));
    commands.insert_resource(ShackAssets { shack_sprites });
  }

  fn spawn_shack(mut commands: Commands, shack_assets: Res<ShackAssets>) {
    commands.spawn(ShackBundle {
      sprite: Sprite::from_image(shack_assets.shack_sprites[0].clone_weak()),
      pos: Position::new(
        WorldVec2::new(
          WorldUnit::RIGHT - Self::WIDTH / 2.,
          WorldUnit::BOTTOM + Self::HEIGHT / 2.,
        ) + WorldVec2::new_normalized(0., 0.2),
        Self::WIDTH,
        Self::IMG_WIDTH,
        Self::Z_IDX,
      ),
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
    let shack_pos = shack.into_inner().pos;
    for (rain_pos, mut rain_vel) in &mut rain_query {
      let rain_pos = rain_pos.pos;
      let diff = rain_pos - shack_pos;

      let tl_corner = shack_pos + WorldVec2::new(-Self::WIDTH / 2., Self::HEIGHT / 2.);
      let from_tl_corner = rain_pos - tl_corner;

      if diff.x.abs() <= Self::WIDTH / 2. && diff.y.abs() <= Self::HEIGHT / 2. {
        if from_tl_corner.y < -from_tl_corner.x {
          if rain_vel.delta.x > WorldUnit::ZERO {
            rain_vel.delta.x = -rain_vel.delta.x * Self::RAIN_RESTITUTION;
          }
        } else if rain_vel.delta.y < WorldUnit::ZERO {
          rain_vel.delta.y = -rain_vel.delta.y * Self::RAIN_RESTITUTION;
          rain_vel.delta.x += WorldUnit::new(0.1);
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
