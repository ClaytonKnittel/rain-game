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
    system::{Commands, Query, Res, ResMut, Resource},
  },
  image::Image,
  math::{primitives::Rectangle, FloatPow, Vec2, Vec3},
  sprite::Sprite,
  time::{Time, Timer, TimerMode},
  transform::components::Transform,
};
use rand::Rng;

use crate::{
  movable::MoveComponent,
  position::Position,
  rain::{Rain, RainBundle},
  win_info::WinInfo,
  world_init::WorldInitPlugin,
};

enum Character {
  Boy,
  Nun,
  OldMan,
  SchoolGirl,
  Baker,
  BearGirl,
  BoyCap,
}

impl Character {
  const fn num_states(&self, npc_assets: &NpcAssets) -> usize {
    match self {
      Self::Boy => npc_assets.boy_sprites.len(),
      Self::Nun => 1,
      Self::OldMan => npc_assets.old_man_sprites.len(),
      Self::SchoolGirl => npc_assets.school_girl_sprites.len(),
      Self::Baker => npc_assets.baker_sprites.len(),
      Self::BearGirl => npc_assets.bear_girl_sprites.len(),
      Self::BoyCap => npc_assets.boy_cap_sprites.len(),
    }
  }

  fn random_character() -> Self {
    match rand::rng().random_range(0..7) {
      0 => Self::Boy,
      1 => Self::Nun,
      2 => Self::OldMan,
      3 => Self::SchoolGirl,
      4 => Self::Baker,
      5 => Self::BearGirl,
      6 => Self::BoyCap,
      _ => unreachable!(),
    }
  }
}

#[derive(Default)]
enum State {
  #[default]
  Dry,
  Wet {
    timer: Timer,
  },
}

impl State {
  const ANGRY_DURATION: Duration = Duration::from_secs(2);

  fn absorb_rain(&mut self) {
    match self {
      Self::Dry => {
        *self = Self::Wet {
          timer: Timer::new(Self::ANGRY_DURATION, TimerMode::Once),
        }
      }
      Self::Wet { .. } => {}
    }
  }

  fn is_wet(&self) -> bool {
    !matches!(self, State::Dry)
  }

  fn tick(&mut self, delta: Duration) {
    match self {
      Self::Dry => {}
      Self::Wet { timer } => {
        timer.tick(delta);
      }
    }
  }

  fn should_despawn(&self) -> bool {
    if let Self::Wet { timer } = self {
      timer.just_finished()
    } else {
      false
    }
  }
}

#[derive(Component)]
#[require(MoveComponent, Transform)]
struct Npc {
  character: Character,
  state: State,
  animation_idx: usize,
  timer: Timer,
}

impl Npc {
  const WALK_SPEED: f32 = 20.;
  const ANIMATION_PERIOD: Duration = Duration::from_millis(250);

  fn new(character: Character) -> Self {
    Self {
      character,
      state: State::Dry,
      animation_idx: 0,
      timer: Timer::new(Self::ANIMATION_PERIOD, TimerMode::Repeating),
    }
  }

  fn current_asset(&self, npc_assets: &NpcAssets) -> Handle<Image> {
    if self.state.is_wet() {
      match self.character {
        Character::Boy => npc_assets.wet_boy_sprite.clone_weak(),
        Character::Nun => npc_assets.wet_nun_sprite.clone_weak(),
        Character::OldMan => npc_assets.wet_old_man_sprite.clone_weak(),
        Character::SchoolGirl => npc_assets.wet_school_girl_sprite.clone_weak(),
        Character::Baker => npc_assets.wet_baker_sprite.clone_weak(),
        Character::BearGirl => npc_assets.wet_bear_girl_sprite.clone_weak(),
        Character::BoyCap => npc_assets.wet_boy_cap_sprite.clone_weak(),
      }
    } else {
      match self.character {
        Character::Boy => npc_assets.boy_sprites[self.animation_idx].clone_weak(),
        Character::Nun => npc_assets.nun_sprite.clone_weak(),
        Character::OldMan => npc_assets.old_man_sprites[self.animation_idx].clone_weak(),
        Character::SchoolGirl => npc_assets.school_girl_sprites[self.animation_idx].clone_weak(),
        Character::Baker => npc_assets.baker_sprites[self.animation_idx].clone_weak(),
        Character::BearGirl => npc_assets.bear_girl_sprites[self.animation_idx].clone_weak(),
        Character::BoyCap => npc_assets.boy_cap_sprites[self.animation_idx].clone_weak(),
      }
    }
  }

  fn tick(&mut self, delta: Duration, npc_assets: &NpcAssets, sprite: &mut Sprite) {
    self.timer.tick(delta);
    if self.timer.just_finished() {
      self.animation_idx = (self.animation_idx + 1) % self.character.num_states(npc_assets);
    }

    self.state.tick(delta);

    sprite.image = self.current_asset(npc_assets);
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

  fn spawn(mut commands: Commands, character: Character, pos: Position, npc_assets: &NpcAssets) {
    let npc = Npc::new(character);
    commands.spawn(NpcBundle {
      sprite: Sprite::from_image(npc.current_asset(npc_assets)),
      npc,
      transform: Transform::from_scale(Vec3::splat(Self::WIDTH / Self::BOY_WIDTH)),
      pos,
    });
  }
}

#[derive(Resource)]
struct NpcAssets {
  boy_sprites: [Handle<Image>; 4],
  wet_boy_sprite: Handle<Image>,
  nun_sprite: Handle<Image>,
  wet_nun_sprite: Handle<Image>,
  old_man_sprites: [Handle<Image>; 2],
  wet_old_man_sprite: Handle<Image>,
  school_girl_sprites: [Handle<Image>; 2],
  wet_school_girl_sprite: Handle<Image>,
  baker_sprites: [Handle<Image>; 2],
  wet_baker_sprite: Handle<Image>,
  bear_girl_sprites: [Handle<Image>; 2],
  wet_bear_girl_sprite: Handle<Image>,
  boy_cap_sprites: [Handle<Image>; 2],
  wet_boy_cap_sprite: Handle<Image>,
}

#[derive(Resource)]
struct NpcPluginState {
  spawn_timer: Timer,
}

impl NpcPluginState {
  const NPC_SPAWN_TIMER: Duration = Duration::from_secs(5);

  fn new() -> Self {
    Self {
      spawn_timer: Timer::new(Self::NPC_SPAWN_TIMER, TimerMode::Repeating),
    }
  }
}

pub struct NpcPlugin;

impl NpcPlugin {
  fn initialize_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let boy_sprites = [1, 2, 3, 2].map(|idx| asset_server.load(format!("boy/boy_{idx}_right.png")));
    let wet_boy_sprite = asset_server.load("boy/boy_wet.png");

    let nun_sprite = asset_server.load("nun/nun_right.png");
    let wet_nun_sprite = asset_server.load("nun/nun_wet.png");

    let old_man_sprites =
      [1, 2].map(|idx| asset_server.load(format!("old_man/old_man_{idx}_right.png")));
    let wet_old_man_sprite = asset_server.load("old_man/old_man_wet.png");

    let school_girl_sprites =
      [1, 2].map(|idx| asset_server.load(format!("school_girl/school_girl_{idx}_right.png")));
    let wet_school_girl_sprite = asset_server.load("school_girl/school_girl_wet.png");

    let baker_sprites = [1, 2].map(|idx| asset_server.load(format!("baker/baker_{idx}.png")));
    let wet_baker_sprite = asset_server.load("baker/wet_baker.png");

    let bear_girl_sprites =
      [1, 2].map(|idx| asset_server.load(format!("bear_girl/bear_girl_{idx}.png")));
    let wet_bear_girl_sprite = asset_server.load("bear_girl/bear_girl_mad.png");

    let boy_cap_sprites = [1, 2].map(|idx| asset_server.load(format!("boy_cap/boy_cap_{idx}.png")));
    let wet_boy_cap_sprite = asset_server.load("boy_cap/boy_cap_mad.png");

    commands.insert_resource(NpcAssets {
      boy_sprites,
      wet_boy_sprite,
      nun_sprite,
      wet_nun_sprite,
      old_man_sprites,
      wet_old_man_sprite,
      school_girl_sprites,
      wet_school_girl_sprite,
      baker_sprites,
      wet_baker_sprite,
      bear_girl_sprites,
      wet_bear_girl_sprite,
      boy_cap_sprites,
      wet_boy_cap_sprite,
    });
    commands.insert_resource(NpcPluginState::new());
  }

  fn spawn_npcs(
    commands: Commands,
    time: Res<Time>,
    mut state: ResMut<NpcPluginState>,
    npc_assets: Res<NpcAssets>,
    win_info: Res<WinInfo>,
  ) {
    state.spawn_timer.tick(time.delta());

    if state.spawn_timer.just_finished() {
      let height = -win_info.height / 4.;
      NpcBundle::spawn(
        commands,
        Character::random_character(),
        Position(Vec2::new(
          -win_info.width / 2. - NpcBundle::WIDTH / 2.,
          height,
        )),
        &npc_assets,
      );
    }
  }

  fn control_npcs(
    mut commands: Commands,
    win_info: Res<WinInfo>,
    mut npc_query: Query<(&mut Npc, &Position, &mut MoveComponent)>,
    rain_query: Query<(Entity, &Position), With<Rain>>,
  ) {
    for (mut npc, &Position(npc_pos), mut npc_vel) in &mut npc_query {
      if npc_pos.x > -win_info.width / 2. + NpcBundle::WIDTH / 2. {
        for (rain_entity, rain_pos) in &rain_query {
          let dist = rain_pos.0 - npc_pos;
          let closest_point = NpcBundle::bounding_rect().closest_point(dist);
          if (closest_point - dist).length_squared() < RainBundle::RADIUS.squared() {
            npc.state.absorb_rain();
            commands.entity(rain_entity).despawn();
          }
        }
      }

      if !npc.state.is_wet() {
        npc_vel.delta = Npc::WALK_SPEED * Vec2::X;
      } else {
        npc_vel.delta = Vec2::ZERO;
      }
    }
  }

  fn npc_tick(
    mut commands: Commands,
    time: Res<Time>,
    npc_assets: Res<NpcAssets>,
    mut query: Query<(Entity, &mut Sprite, &mut Npc)>,
  ) {
    for (entity, mut sprite, mut npc) in &mut query {
      npc.tick(time.delta(), &npc_assets, &mut sprite);

      if npc.state.should_despawn() {
        commands.entity(entity).despawn();
      }
    }
  }
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Startup,
        Self::initialize_plugin.after(WorldInitPlugin::world_init),
      )
      .add_systems(FixedUpdate, (Self::control_npcs, Self::spawn_npcs))
      .add_systems(Update, Self::npc_tick);
  }
}
