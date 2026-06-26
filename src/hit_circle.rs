use bevy::prelude::*;
use serde::Deserialize;

pub const APPROACH_CIRCLE_INITIAL_SCALE: f32 = 2.0;

#[derive(Component)]
pub struct HitCircle;

#[derive(Component)]
pub struct ApproachCircle;

/// Approach rate is stored in milliseconds
#[derive(Component)]
pub struct ApproachRate(u32);

pub struct HitCirclePlugin;

#[derive(Deserialize, Clone, Copy)]
pub struct HitCircleInfo {
    x: f32,
    y: f32,
    /// The time the circle appears relative to the beginning of the map
    appear_time: u64,
    approach_rate: u32,
}

impl Plugin for HitCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shrink_approach_circles);
    }
}

pub fn spawn_circle(
    circle_info: HitCircleInfo,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn((
            HitCircle,
            Transform::from_xyz(circle_info.x, circle_info.y, 0.0),
            Sprite {
                image: asset_server.load("hitcircle.png"),
                color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                ..Default::default()
            },
        ))
        .with_child(Sprite::from_image(asset_server.load("default-1.png")))
        .with_child(Sprite::from_image(
            asset_server.load("hitcircleoverlay.png"),
        ))
        .with_child((
            ApproachCircle,
            ApproachRate(circle_info.approach_rate),
            Sprite::from_image(asset_server.load("approachcircle.png")),
            // NOTE: the z axis does nothing in 2d, just splatting for consistency/brevity
            Transform::default().with_scale(Vec3::splat(APPROACH_CIRCLE_INITIAL_SCALE)),
        ));
}

fn shrink_approach_circles(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &ApproachRate, Entity), With<ApproachCircle>>,
    time: Res<Time>,
) {
    for (mut transform, ar, entity) in &mut query {
        // the difference between initial approach circle size and hit circle size
        let scale_initial_delta = APPROACH_CIRCLE_INITIAL_SCALE - 1.0;

        // if this u128 -> u32 conversion ever breaks holy your frames are cooked
        let transform_delta: f32 =
            scale_initial_delta / (ar.0 as f32) * time.delta().as_millis() as f32;

        transform.scale -= transform_delta;

        // NOTE: this relies on the circle being shrunk uniformly
        if transform.scale.x <= 0.95 {
            commands.entity(entity).despawn();
        }
    }
}
