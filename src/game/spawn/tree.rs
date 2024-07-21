use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

use super::level::{CellPosition, CELL_SIZE};

#[derive(Component, Default, Debug, Reflect)]
pub struct Tree {
    life: f32,
    reproduced: bool,
}

const TREE_MAX_LIFE: f32 = 10.0; //seconds

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Tree>();
    app.add_systems(
        Update,
        (tree_age, tree_mature, tree_die, tree_spawn).run_if(in_state(Screen::Playing)),
    );
}

fn tree_age(mut query: Query<&mut Tree>, time: Res<Time>) {
    for mut tree in &mut query {
        tree.life += time.delta_seconds();
    }
}

fn tree_mature(
    mut query: Query<(&Tree, &mut Transform, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (tree, mut _transform, mut material) in &mut query {
        let percent = (tree.life / TREE_MAX_LIFE).clamp(0.5, 1.0);

        //transform.scale = Vec3::splat(percent);
        *material = materials.add(Color::srgb(percent, percent, 0.6));
    }
}

fn tree_die(mut commands: Commands, query: Query<(Entity, &Tree)>) {
    for (entity, tree) in &query {
        if tree.life > TREE_MAX_LIFE {
            commands.entity(entity).despawn();
        }
    }
}

fn tree_spawn(
    mut commands: Commands,
    mut query: Query<(&CellPosition, &mut Tree)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (position, mut tree) in &mut query {
        if !tree.reproduced && tree.life > TREE_MAX_LIFE / 2.0 {
            let rng = rand::thread_rng().gen_range(0..=100);

            if rng > 60 {
                tree.reproduced = true;
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cylinder::new(CELL_SIZE / 2.0, CELL_SIZE * 2.0)),
                        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                        ..default()
                    },
                    CellPosition {
                        // This does not check if there is already a tree there
                        x: position.x + rand::thread_rng().gen_range(-2..=2),
                        y: position.y + rand::thread_rng().gen_range(-2..=2),
                    },
                    Tree::default(),
                    StateScoped(Screen::Playing),
                ));
            }
        }
    }
}
