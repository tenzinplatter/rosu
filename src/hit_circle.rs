use bevy::prelude::*;
use serde::Deserialize;

use crate::callback::Callback;

const APPROACH_CIRCLE_INITIAL_SCALE: f32 = 2.0;
const INITIAL_OPACITY: f32 = 0.0;
const FINAL_OPACITY: f32 = 0.7;
const FADE_IN_TIME_MS: u32 = 300;

#[derive(Component)]
pub struct HitCircle;

#[derive(Component)]
pub struct ApproachCircle;

/// Approach rate is stored in milliseconds
#[derive(Component)]
pub struct ApproachRate(u32);

#[derive(Component)]
struct FadingIn;

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
        app.add_systems(Update, (shrink_approach_circles, fade_in_hit_circles));
    }
}

impl HitCircle {
    pub(crate) fn cursor_colliding(cursor_pos: Vec3, t: &Transform) -> bool {
        const CURSOR_HITBOX_RADIUS: f32 = 10.0;
        const HIT_CIRCLE_HITBOX_RADIUS: f32 = 20.0;
        cursor_pos.distance(t.translation) <= CURSOR_HITBOX_RADIUS + HIT_CIRCLE_HITBOX_RADIUS
    }
}

pub fn spawn_circle(circle_info: HitCircleInfo, commands: &mut Commands) {
    let timer = Timer::from_seconds(circle_info.appear_time as f32 / 1000.0, TimerMode::Once);
    // register callback to fade in circle
    let system_id = commands.register_system(
        move |mut commands: Commands, asset_server: Res<'_, AssetServer>| {
            commands
                .spawn((
                    HitCircle,
                    FadingIn,
                    Transform::from_xyz(circle_info.x, circle_info.y, 0.0),
                    Sprite {
                        image: asset_server.load("hitcircle.png"),
                        color: Color::srgba(1.0, 1.0, 1.0, INITIAL_OPACITY),
                        ..Default::default()
                    },
                ))
                .with_child(Sprite::from_image(asset_server.load("default-1.png")))
                .with_child(Sprite::from_image(
                    asset_server.load("hitcircleoverlay.png"),
                ))
                .with_child((
                    ApproachCircle,
                    FadingIn,
                    ApproachRate(circle_info.approach_rate),
                    // NOTE: the z axis does nothing in 2d, just splatting for consistency/brevity
                    Transform::default().with_scale(Vec3::splat(APPROACH_CIRCLE_INITIAL_SCALE)),
                    Sprite {
                        image: asset_server.load("approachcircle.png"),
                        color: Color::srgba(1.0, 1.0, 1.0, INITIAL_OPACITY),
                        ..Default::default()
                    },
                ));
        },
    );

    commands.spawn(Callback { timer, system_id });
}

fn fade_in_hit_circles(
    mut hit_circles: Query<(&mut Sprite, Entity), With<FadingIn>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let alpha_step = (FINAL_OPACITY - INITIAL_OPACITY) / (FADE_IN_TIME_MS as f32 / 1000.)
        * time.delta().as_secs_f32();

    for (mut sprite, hit_circle) in &mut hit_circles {
        let alpha = &mut sprite.color.to_srgba().alpha;
        if *alpha + alpha_step < FINAL_OPACITY {
            sprite.color.set_alpha(*alpha + alpha_step);
        } else {
            commands.entity(hit_circle).remove::<FadingIn>();
            sprite.color.set_alpha(FINAL_OPACITY);
        }
    }
}

type ShrinkQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Transform,
        &'static ApproachRate,
        &'static ChildOf,
        Entity,
    ),
    With<ApproachCircle>,
>;

fn shrink_approach_circles(mut commands: Commands, mut query: ShrinkQuery, time: Res<Time>) {
    for (mut transform, ar, child_of, approach_circle) in &mut query {
        // the difference between initial approach circle size and hit circle size
        let scale_initial_delta = APPROACH_CIRCLE_INITIAL_SCALE - 1.0;

        // if this u128 -> u32 conversion ever breaks holy your frames are cooked
        let transform_delta: f32 =
            scale_initial_delta / (ar.0 as f32) * time.delta().as_millis() as f32;

        transform.scale -= transform_delta;

        // NOTE: this relies on the circle being shrunk uniformly
        if transform.scale.x <= 0.95 {
            commands.entity(approach_circle).despawn();
            let hit_circle = child_of.parent();
            commands.entity(hit_circle).despawn_children();
            commands.entity(hit_circle).despawn();
        }
    }
}
