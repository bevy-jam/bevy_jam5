//! Spawn the main level by triggering other observers.

use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};

use crate::screen::Screen;

use super::tree::Tree;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.add_systems(
        Update,
        (cell_position_to_transform, draw_level).run_if(in_state(Screen::Playing)),
    );
    app.register_type::<CellPosition>();
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    //commands.trigger(SpawnPlayer);

    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: -1,
                ..default()
            },
            projection: OrthographicProjection {
                // 6 world units per window height.
                scaling_mode: ScalingMode::FixedVertical(6.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = true;

    // trees
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(CELL_SIZE / 2.0, CELL_SIZE * 2.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            ..default()
        },
        CellPosition { x: 0, y: 0 },
        Tree::default(),
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(CELL_SIZE / 2.0, CELL_SIZE * 2.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            ..default()
        },
        CellPosition { x: 1, y: 0 },
        Tree::default(),
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(CELL_SIZE / 2.0, CELL_SIZE * 2.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            ..default()
        },
        CellPosition { x: 0, y: 1 },
        Tree::default(),
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(CELL_SIZE / 2.0, CELL_SIZE * 2.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            ..default()
        },
        CellPosition { x: -1, y: -1 },
        Tree::default(),
        StateScoped(Screen::Playing),
    ));

    // light
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 5.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}

pub const CELL_SIZE: f32 = 0.5;
pub const CELL_ROTATION: f32 = PI / 2.;
pub const CELL_COUNT: u32 = 100;

fn draw_level(mut gizmos: Gizmos) {
    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(CELL_ROTATION),
        UVec2::splat(CELL_COUNT),
        Vec2::new(CELL_SIZE, CELL_SIZE),
        // Light gray
        LinearRgba::gray(0.2),
    );
}

#[derive(Component, Reflect, Default, Debug)]
pub struct CellPosition {
    pub x: i32,
    pub y: i32,
}

fn cell_position_to_transform(
    mut query: Query<(&CellPosition, &mut Transform), Changed<CellPosition>>,
) {
    for (cell_position, mut transform) in &mut query {
        *transform = Transform::from_xyz(
            CELL_SIZE / 2.0 + cell_position.x as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE / 2.0 + cell_position.y as f32 * CELL_SIZE,
        );
    }
}
