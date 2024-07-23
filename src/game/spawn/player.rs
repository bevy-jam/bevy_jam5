//! Spawn the player.

use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::game::character_controller::CharacterControllerBundle;
use crate::{
    game::{animation::PlayerAnimation, assets::ImageAssets, physics::GravityController},
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

#[derive(Component, Copy, Clone)]
pub struct PlayerSpeed(pub f32);

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
        PlayerSpeed(1.),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(8.))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., 1050., 0.),
            ..default()
        },
        // SpriteBundle {
        //     texture: image_assets.ducky.clone_weak(),
        //     transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
        //     ..Default::default()
        // },
        // TextureAtlas {
        //     layout: texture_atlas_layout.clone(),
        //     index: player_animation.get_atlas_index(),
        // },
        // player_animation,
        CharacterControllerBundle::new(Collider::circle(8.0 as Scalar)).with_movement(
            2000.0,
            0.9,
            250.0,
            std::f32::consts::PI * 0.45,
        ),
        // GravityScale(1.),
        // GravityController(10000.0),
        StateScoped(Screen::Playing),
    ));
}

//TODO: Maybe it is better to spawn a new camera as child of player
fn camera_follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    //TODO: This breaks going back to the menu
    //TODO: Also does not consider rotation
    let player_transform = player_transform.single();

    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    let mut angle = Vec3::Y.angle_between(player_transform.translation);
    if player_transform.translation.x > 0. {
        angle = std::f32::consts::PI * 2. - angle;
    }

    camera_transform.rotation = Quat::from_rotation_z(angle);
}
