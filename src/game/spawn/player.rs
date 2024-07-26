//! Spawn the player.

use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::game::character_controller::CharacterControllerBundle;
use crate::{
    game::{animation::PlayerAnimation, assets::ImageAssets},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();

    app.add_systems(
        FixedPostUpdate,
        camera_follow_player
            .run_if(in_state(Screen::Playing))
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
    );
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;


fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(8.))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., 20100.0, 0.),
            ..default()
        },
        CharacterControllerBundle::new(Collider::circle(8.0 as Scalar)).with_movement(
            2000.0,
            0.98,
            500.0,
            std::f32::consts::PI * 0.45,
        ),
        StateScoped(Screen::Playing),
    ));
}

//TODO: Maybe it is better to spawn a new camera as child of player
fn camera_follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    //TODO: This breaks going back to the menu (I'm not sure what it means)
    let player_transform = player_transform.single();

    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
