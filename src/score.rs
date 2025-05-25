use bevy::{
  app::{App, FixedUpdate, Plugin, Startup, Update},
  asset::AssetServer,
  ecs::{
    bundle::Bundle,
    component::Component,
    event::{Event, EventReader},
    query::With,
    schedule::IntoSystemConfigs,
    system::{Commands, Res, ResMut, Resource, Single},
  },
  text::{JustifyText, TextFont, TextLayout},
  ui::{widget::Text, Node, PositionType, Val},
  utils::default,
};

use bevy_world_space::world_init::WorldInitPlugin;

#[derive(Event)]
pub struct EarnPoint;

#[derive(Default, Resource)]
struct ScoreResource {
  points: u32,
}

#[derive(Component)]
struct Score;

#[derive(Bundle)]
pub struct ScoreBundle {
  text: Text,
  font: TextFont,
  layout: TextLayout,
  node: Node,
  score: Score,
}

pub struct ScorePlugin;

impl ScorePlugin {
  fn initialize_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ScoreBundle {
      text: Text::new("Score: 0"),
      font: TextFont {
        font: asset_server.load("fonts/Rubik-VariableFont_wght.ttf"),
        font_size: 67.0,
        ..default()
      },
      layout: TextLayout::new_with_justify(JustifyText::Center),
      node: Node {
        position_type: PositionType::Absolute,
        top: Val::Px(5.0),
        right: Val::Px(5.0),
        ..default()
      },
      score: Score,
    });
  }

  fn earn_points(mut points: EventReader<EarnPoint>, mut score: ResMut<ScoreResource>) {
    score.points += points.read().count() as u32;
  }

  fn update_score(mut score_ui: Single<&mut Text, With<Score>>, score: Res<ScoreResource>) {
    score_ui.0 = format!("Score: {}", score.points);
  }
}

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<EarnPoint>()
      .insert_resource(ScoreResource::default())
      .add_systems(
        Startup,
        ScorePlugin::initialize_ui.after(WorldInitPlugin::world_init),
      )
      .add_systems(FixedUpdate, ScorePlugin::earn_points)
      .add_systems(Update, ScorePlugin::update_score);
  }
}
