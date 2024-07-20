//! Spawn the player.

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs

    commands.spawn((
        Name::new("Player"),
        Player,
        MovementController::default(),
        Movement { speed: 10.0 },
        WrapWithinWindow,
        StateScoped(Screen::Playing),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(4.))),
            material: materials.add(Color::srgba(1.,0.2,0.05,1.)),
            transform: Transform::from_translation(Vec3::new(0.,0.,1.)),
            ..default()
        },
    ));
}
