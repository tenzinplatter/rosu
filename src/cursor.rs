use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::hit_circle::HitCircle;

#[derive(Component)]
struct Cursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor)
            .add_systems(Update, (move_cursor, handle_clicks));
    }
}

fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Single<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
) {
    let cursor_options = &mut query.1;
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Confined;

    let window = query.0;
    let cursor_pos = get_cursor_pos(window);
    commands.spawn((
        Cursor,
        Sprite::from_image(asset_server.load("cursor.png")),
        Transform::from_translation(cursor_pos.unwrap_or_default()),
    ));
}

// TODO: add keys
fn handle_clicks(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    hit_circles: Query<(Entity, &Transform), With<HitCircle>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let cursor = get_cursor_pos(&window).unwrap_or_default();
        for (circle, transform) in &hit_circles {
            if HitCircle::cursor_colliding(cursor, transform) {
                commands.entity(circle).despawn_children();
                commands.entity(circle).despawn();
            }
        }
    }
}

fn move_cursor(
    mut cursor: Single<&mut Transform, With<Cursor>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let cursor_pos = get_cursor_pos(&window).unwrap_or_default();
    cursor.translation = cursor_pos;
}

fn get_cursor_pos(window: &Window) -> Option<Vec3> {
    let offset_vec = window.size() / 2.0;
    let cursor_logical = window.cursor_position()? - offset_vec;
    let cursor_xy = cursor_logical.with_y(-cursor_logical.y) / window.scale_factor();
    Some(Vec3 {
        x: cursor_xy.x,
        y: cursor_xy.y,
        z: 0.0,
    })
}
