use std::time::Duration;

use bevy::{
  app::{App, FixedUpdate, Plugin, Startup, Update},
  asset::{AssetServer, Handle},
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, Resource},
  },
  image::Image,
  math::{primitives::Rectangle, FloatPow, Vec2, Vec3},
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
};

use crate::{
  movable::MoveComponent,
  position::Position,
  rain::{Rain, RainBundle},
  win_info::WinInfo,
  world_init::WorldInitPlugin,
};

#[derive(Default)]
enum Wetness {
  #[default]
  Dry,
  Wet {
    level: u8,
  },
  Soaked,
}

impl Wetness {
  const MAX_WETNESS: u8 = 3;

  fn absorb_rain(&mut self) {
    match self {
      Self::Dry => *self = Self::Wet { level: 1 },
      Self::Wet { level } => {
        if *level == Self::MAX_WETNESS {
          *self = Self::Soaked;
        } else {
          *self = Self::Wet { level: *level + 1 };
        }
      }
      Self::Soaked => {}
    }
  }

  fn is_wet(&self) -> bool {
    !matches!(self, Wetness::Dry)
  }
}

#[derive(Component)]
#[require(MoveComponent, Transform)]
struct Npc {
  wetness: Wetness,
  animation_idx: usize,
  timer: Timer,
}

impl Npc {
  const WALK_SPEED: f32 = 20.;

  fn new(timeout: Duration) -> Self {
    Self {
      wetness: Wetness::Dry,
      animation_idx: 0,
      timer: Timer::new(timeout, TimerMode::Repeating),
    }
  }
}

#[derive(Bundle)]
struct NpcBundle {
  sprite: Sprite,
  npc: Npc,
  transform: Transform,
  pos: Position,
}

impl NpcBundle {
  // const BOY_WIDTH: f32 = 750.;
  // const BOY_HEIGHT: f32 = 1250.;
  const BOY_WIDTH: f32 = 589.;
  const BOY_HEIGHT: f32 = 656.;
  const ASPECT_RATIO: f32 = Self::BOY_HEIGHT / Self::BOY_WIDTH;

  const WIDTH: f32 = 60.;
  const HEIGHT: f32 = Self::WIDTH * Self::ASPECT_RATIO;

  fn bounding_rect() -> Rectangle {
    Rectangle::new(Self::WIDTH, Self::HEIGHT)
  }

  fn spawn(mut commands: Commands, pos: Position, image: Handle<Image>) {
    commands.spawn(NpcBundle {
      sprite: Sprite::from_image(image),
      npc: Npc::new(Duration::from_millis(250)),
      transform: Transform::from_scale(Vec3::splat(Self::WIDTH / Self::BOY_WIDTH)),
      pos,
    });
  }
}

#[derive(Resource)]
struct NpcAssets {
  boy_sprites: [Handle<Image>; 4],
  wet_boy_sprite: Handle<Image>,
}

pub struct NpcPlugin;

impl NpcPlugin {
  const SIGHT_DIST: f32 = 200.;

  fn initialize_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let boy_sprites = [1, 2, 3, 2].map(|idx| asset_server.load(format!("boy/boy_{idx}_right.png")));
    let wet_boy_sprite = asset_server.load("boy/boy_wet.png");

    commands.insert_resource(NpcAssets { boy_sprites, wet_boy_sprite });
  }

  fn spawn_npc(commands: Commands, npc_assets: Res<NpcAssets>, win_info: Res<WinInfo>) {
    let height = -win_info.height / 4.;
    NpcBundle::spawn(
      commands,
      Position(Vec2::new(
        -win_info.width / 2. - NpcBundle::WIDTH / 2.,
        height,
      )),
      npc_assets.boy_sprites[0].clone_weak(),
    );
  }

  fn find_nearest_visible_rain_pos(
    npc_pos: Vec2,
    rain: impl IntoIterator<Item = (Entity, Vec2)>,
  ) -> Option<(Entity, Vec2)> {
    rain
      .into_iter()
      .fold(None, |nearest_rain, (entity, rain_pos)| {
        let dist = (rain_pos - npc_pos).length_squared();
        if dist < Self::SIGHT_DIST.squared() {
          Some(
            nearest_rain
              .map(|(nearest_entity, nearest_rain_pos)| {
                if dist < (nearest_rain_pos - npc_pos).length_squared() {
                  (entity, rain_pos)
                } else {
                  (nearest_entity, nearest_rain_pos)
                }
              })
              .unwrap_or((entity, rain_pos)),
          )
        } else {
          nearest_rain
        }
      })
  }

  fn control_npcs(
    mut commands: Commands,
    mut npc_query: Query<(&mut Npc, &Position, &mut MoveComponent)>,
    rain_query: Query<(Entity, &Position), With<Rain>>,
  ) {
    for (mut npc, &Position(npc_pos), mut npc_vel) in &mut npc_query {
      npc_vel.delta = Npc::WALK_SPEED * Vec2::X;

      let nearest_rain = Self::find_nearest_visible_rain_pos(
        npc_pos,
        rain_query
          .iter()
          .map(|(entity, rain_pos)| (entity, rain_pos.0)),
      );

      if let Some((rain_entity, nearest_rain)) = nearest_rain {
        let dist = nearest_rain - npc_pos;
        let closest_point = NpcBundle::bounding_rect().closest_point(dist);
        if (closest_point - dist).length_squared() < RainBundle::RADIUS.squared() {
          npc.wetness.absorb_rain();
          commands.entity(rain_entity).despawn();
        }
      }
    }
  }

  fn set_npc_wetness(
    time: Res<Time>,
    npc_assets: Res<NpcAssets>,
    mut query: Query<(&mut Sprite, &mut Npc)>,
  ) {
    for (mut sprite, mut npc) in &mut query {
      if npc.wetness.is_wet() {
        sprite.image = npc_assets.wet_boy_sprite.clone_weak();
      } else {
        npc.timer.tick(time.delta());
        if npc.timer.just_finished() {
          npc.animation_idx = (npc.animation_idx + 1) % npc_assets.boy_sprites.len();
        }
        sprite.image = npc_assets.boy_sprites[npc.animation_idx].clone_weak();
      }
    }
  }
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Startup,
        (Self::initialize_plugin, Self::spawn_npc)
          .chain()
          .after(WorldInitPlugin::world_init),
      )
      .add_systems(FixedUpdate, Self::control_npcs)
      .add_systems(Update, Self::set_npc_wetness);
  }
}
