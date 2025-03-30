use bevy::{
  app::{App, FixedUpdate, Plugin, Startup, Update},
  color::Color,
  ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::With,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res},
    world::World,
  },
  hierarchy::{BuildChildren, ChildBuild, Parent},
  math::{primitives::Rectangle, FloatPow, Vec2},
  transform::components::Transform,
};

use crate::{
  color::{ColorComponent, StrictColor},
  movable::MoveComponent,
  position::Position,
  rain::{Rain, RainBundle},
  screen_object::ScreenObjectBundle,
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

  fn color(&self) -> StrictColor {
    match self {
      Self::Dry => StrictColor::new(0xEB, 0xBF, 0x6E),
      Self::Wet { level: 1 } => StrictColor::new(0xEB, 0xAD, 0x6E),
      Self::Wet { level: 2 } => StrictColor::new(0xEB, 0x96, 0x6D),
      Self::Wet { level: 3 } => StrictColor::new(0xEB, 0x80, 0x6C),
      Self::Soaked => StrictColor::new(0xEB, 0x6C, 0x73),
      _ => unreachable!(),
    }
  }
}

#[derive(Component, Default)]
#[require(MoveComponent, Transform)]
struct Npc {
  wetness: Wetness,
}

impl Npc {
  const WALK_SPEED: f32 = 20.;
}

#[derive(Bundle)]
struct NpcBundle {
  npc: Npc,
  pos: Position,
}

#[derive(Component)]
struct NpcBody;

#[derive(Bundle)]
struct NpcBodyBundle {
  screen_object: ScreenObjectBundle,
  npc_body: NpcBody,
  pos: Position,
}

#[derive(Component)]
struct NpcEye;

#[derive(Bundle)]
struct NpcEyeBundle {
  screen_object: ScreenObjectBundle,
  npc_eye: NpcEye,
  pos: Position,
}

impl NpcBundle {
  const WIDTH: f32 = 50.;
  const HEIGHT: f32 = 80.;

  fn bounding_rect() -> Rectangle {
    Rectangle::new(Self::WIDTH, Self::HEIGHT)
  }

  fn spawn(mut commands: Commands, pos: Position) {
    commands.queue(|world: &mut World| {
      let body_screen_object = ScreenObjectBundle::new(
        Self::bounding_rect(),
        Color::srgb(0.8, 0.7, 0.6),
        1.0,
        world,
      );
      let eye_screen_object = ScreenObjectBundle::new(
        Rectangle::new(Self::WIDTH / 4., Self::WIDTH / 4.),
        Color::srgb(0.1, 0.4, 0.4),
        2.0,
        world,
      );

      world
        .spawn(Self { npc: Npc::default(), pos })
        .with_children(move |parent| {
          parent.spawn(NpcBodyBundle {
            screen_object: body_screen_object,
            npc_body: NpcBody,
            pos: Position(Vec2::ZERO),
          });

          parent.spawn(NpcEyeBundle {
            screen_object: eye_screen_object,
            npc_eye: NpcEye,
            pos: Position(Vec2 {
              x: Self::WIDTH / 4.,
              y: Self::HEIGHT * 0.4,
            }),
          });
        });
    });
  }
}

pub struct NpcPlugin;

impl NpcPlugin {
  const SIGHT_DIST: f32 = 200.;

  fn spawn_npc(commands: Commands, win_info: Res<WinInfo>) {
    let height = -win_info.height / 4.;
    NpcBundle::spawn(
      commands,
      Position(Vec2::new(
        -win_info.width / 2. - NpcBundle::WIDTH / 2.,
        height,
      )),
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

  fn set_npc_colors(
    npc_query: Query<&Npc>,
    mut eye_query: Query<(&mut ColorComponent, &Parent), With<NpcBody>>,
  ) {
    for (mut body_color, parent) in &mut eye_query {
      let npc = npc_query.get(parent.get()).unwrap();
      let color = npc.wetness.color();

      body_color.set_color(color);
    }
  }
}

impl Plugin for NpcPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, Self::spawn_npc.after(WorldInitPlugin::world_init))
      .add_systems(FixedUpdate, Self::control_npcs)
      .add_systems(Update, Self::set_npc_colors);
  }
}
