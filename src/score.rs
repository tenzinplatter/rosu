use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score_text)
            .add_systems(Update, increment_score);
    }
}

#[derive(Component)]
struct Score(f32);

fn setup_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/InconsolataNerdFont-Regular.ttf");
    commands.spawn((
        Score(0.0),
        Text::new("0"),
        TextFont {
            font: font.clone().into(),
            font_size: FontSize::Px(42.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            right: px(5),
            ..default()
        },
    ));
}

fn increment_score(score_text: Single<(&mut Score, &mut Text), With<Score>>) {
    let (mut score, mut span) = score_text.into_inner();
    score.0 += 1000.0;
    **span = format!("{:0>6}", score.0);
}
