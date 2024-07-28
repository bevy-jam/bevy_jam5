use avian2d::prelude::*;
use bevy::{ecs::system::QueryLens, prelude::*, window::PrimaryWindow};

use super::SelectedObject;

// todo: I'm not entirely sure if it's ok to not including `With<Object>` here
pub fn find_closest_object(
    mut object_query: QueryLens<(&GlobalTransform, &Collider, Entity)>,
    world_cursor: Vec2,
    max_select_distance: f32,
) -> Option<Entity> {
    let mut min_dist = f32::MAX;
    let mut closest = None;

    // todo: optimize with aabb?
    for (global_transform, collider, entity) in object_query.query().iter() {
        let dist =
            collider.distance_to_point(global_transform, global_transform, world_cursor, false);
        // info!("dist({entity}) = {dist}");
        if dist < min_dist && dist < max_select_distance {
            closest = Some(entity)
        }
    }

    closest
}

#[deprecated]
pub fn select_closest_object(
    mut object_query: QueryLens<(&GlobalTransform, &Collider, Entity)>,
    world_cursor: Vec2,
    max_select_distance: f32,
    mut selected_object: &mut ResMut<SelectedObject>,
) {
    if let Some(entity) = find_closest_object(object_query, world_cursor, max_select_distance) {
        selected_object.select(entity)
    }
}

#[derive(Resource, Reflect, Default)]
pub struct WorldCursor(pub Option<Vec2>);

pub fn update_world_cursor(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut world_cursor: ResMut<WorldCursor>,
) {
    if let Ok(window) = window_query.get_single() {
        let Some(cursor) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        let pos = camera
            .viewport_to_world(camera_transform, cursor)
            .map(|ray| {
                // info!("cursor world ray: {ray:?}"); // I'm curious what is the z coordinate of this, for me it's equal to 500.0...9
                ray.origin.truncate()
            });

        world_cursor.0 = pos;
    }
}
