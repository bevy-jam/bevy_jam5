//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<ColorMaterial>>) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let mut map = ForestMap::new((100,80));
    map.spawn_entities(&mut commands, &mut meshes, &mut materials);
    commands.insert_resource(map);
    commands.trigger(SpawnPlayer);
}

#[derive(Bundle)]
pub struct ForestTileBundle {
    tile: ForestTile,
    mm2db: MaterialMesh2dBundle<ColorMaterial>
}

impl ForestTileBundle {
    pub fn new(pos: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {
        let tile = ForestTile::default();
        Self {
            tile,
            mm2db: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.,1.))),
                material: materials.add(Color::srgba(0.,1.,0.2,tile.growth)),
                transform: Transform::from_translation(pos.extend(0.)),
                    ..default()
            },
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct ForestTile {
    pub growth: f32
}

impl Default for ForestTile {
    fn default() -> Self {
        Self { growth: 1. }
    }
}


#[derive(Debug, Resource)]
pub struct ForestMap {
    map: Vec<Entity>,
    pub dimensions: (usize, usize)
}

impl ForestMap {
    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        self.map.get(x * self.dimensions.0 + y).copied()
    }

    pub fn new(dimensions: (usize, usize)) -> Self {
        Self {
            map: vec![],
            dimensions,
        }
    }
    
    fn spawn_entities(&mut self,commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                let pos = Vec2::new(i as f32, j as f32);
                let e = commands.spawn(ForestTileBundle::new(pos, meshes, materials)).id();
                self.map.push(e);
            }
        }
    }
}