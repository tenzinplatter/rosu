use bevy::{prelude::*, window::PrimaryWindow};
use uuid::Uuid;

use crate::file_parser::{Map, MapInfo};

const LOGO_INITIAL_SCALE: f32 = 0.4;
const LOGO_PULSE_DELTA: f32 = 0.05;
const LOGO_PULSE_TIME_MS: f32 = 400.0;

pub struct UIPlugin;

#[derive(States, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UIState {
    EntryMenu,
    SongSelect,
}

struct UISongSelectState {
    selected_song_id: Option<Uuid>,
    scroll_position: usize,
}

#[derive(Component)]
struct SongListContainer;

#[derive(Component)]
struct SpawnSongBoxRequest {
    pos: Vec2,
    size: Vec2,
}

#[derive(Resource, Default)]
struct SongCursor {
    index: Option<usize>,
    song_id: Option<Uuid>,
}

#[derive(Component)]
struct Logo;

#[derive(Component, Clone, Copy)]
enum LogoPulseDirection {
    Growing,
    Shrinking,
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIState::EntryMenu)
            .add_systems(Startup, init_song_cursor)
            .add_systems(
                Update,
                (
                    (draw_song_select, spawn_song_boxes)
                        .chain()
                        .run_if(in_state(UIState::SongSelect)),
                    (draw_entry_menu, pulse_logo).run_if(in_state(UIState::EntryMenu)),
                ),
            );
    }
}

fn init_song_cursor(mut commands: Commands) {
    commands.insert_resource(SongCursor::default());
}

fn pulse_logo(
    logo: Single<(&mut Transform, &mut LogoPulseDirection), With<Logo>>,
    time: Res<Time>,
) {
    let (mut t, mut dir) = logo.into_inner();
    let is_growing = matches!(*dir, LogoPulseDirection::Growing);
    let growing_speed = if is_growing { 10.0 } else { 1.0 };
    let size_dt = (time.delta().as_millis() as f32 / LOGO_PULSE_TIME_MS)
        * LOGO_PULSE_DELTA
        * growing_speed;

    if is_growing {
        if t.scale.x >= LOGO_INITIAL_SCALE {
            t.scale -= size_dt;
            *dir = LogoPulseDirection::Shrinking;
        } else {
            t.scale += size_dt;
        }
    } else {
        if t.scale.x <= LOGO_INITIAL_SCALE - LOGO_PULSE_DELTA {
            t.scale += size_dt;
            *dir = LogoPulseDirection::Growing;
        } else {
            t.scale -= size_dt;
        }
    }
}

fn draw_entry_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    logo: Option<Single<&Logo>>,
) {
    if logo.is_some() {
        return;
    }

    let logo = (
        Logo,
        LogoPulseDirection::Shrinking,
        Sprite::from_image(asset_server.load("lazer.png")),
        Transform::default().with_scale(Vec3::splat(LOGO_INITIAL_SCALE)),
    );
    commands.spawn(logo);
}

fn draw_song_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    songs: Query<&MapInfo, With<Map>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let size = Vec2::new(300.0, 100.0);
    let mut pos = Vec2::new(window.size().x - size.x, 0.0);
    for song in &songs {
        commands.spawn(SpawnSongBoxRequest { pos, size });
        pos.y += size.y;
    }
}

fn spawn_song_boxes(
    mut commands: Commands,
    requests: Query<&SpawnSongBoxRequest>,
    parent_container: Option<Single<&Node, With<SongListContainer>>>,
) {
    for req in &requests {
        spawn_box_at(&mut commands, req.pos);
    }
}

fn spawn_box_at(mut commands: &mut Commands, pos: Vec2) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };
    let square = (
        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        Node {
            width: Val::Px(200.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
    );
    commands.spawn(((container, SongListContainer), children![(square)]));
}
