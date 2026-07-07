use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use anyhow::Context;
use bevy::prelude::*;
use expand_tilde::expand_tilde;

use crate::hit_circle::HitCircleInfo;

mod general;

pub struct MapPlugin;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct MapInfo {
    pub name: String,
}

#[derive(Component)]
struct HitCircles(Vec<HitCircleInfo>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, parse_all_maps);
    }
}

fn parse_all_maps(mut commands: Commands) -> Result {
    let map_dir =
        expand_tilde("~/.rosu/maps").context("Failed to expand map dir path at ~/.rosu/maps")?;
    let entries = read_dir(map_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            match parse_map_file(&path) {
                Ok(info) => add_map_to_ecs(&mut commands, info),
                Err(e) => tracing::warn!("Failed to read map at {}: {}", path.display(), e),
            }
        }
    }

    Ok(())
}

fn parse_map_file(map: &Path) -> anyhow::Result<Vec<HitCircleInfo>> {
    let contents = read_to_string(map)?;
    let map_info = serde_json::from_str::<Vec<HitCircleInfo>>(&contents)?;
    Ok(map_info)
}

fn add_map_to_ecs(commands: &mut Commands, map_info: Vec<HitCircleInfo>) {
    commands.spawn((
        Map,
        MapInfo {
            name: "map!".to_string(),
        },
        HitCircles(map_info),
    ));
}
