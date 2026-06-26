use bevy::prelude::*;
use rosu::{CursorPlugin, HitCirclePlugin, MapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rosu".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_plugins(HitCirclePlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(MapPlugin)
        .run();
}
