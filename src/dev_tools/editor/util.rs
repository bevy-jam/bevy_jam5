use avian2d::prelude::*;
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::dev_tools::editor::SELECT_DISTANCE;

use super::SelectedObject;


pub fn find_closest_object(
    mut polyobject_query: QueryLens<(&GlobalTransform, &Collider, Entity)>,
    world_cursor: Vec2,
) -> Option<Entity> {
    let mut min_dist = f32::MAX;
    let mut closest = None;

    // todo: optimize with aabb?
    for (global_transform, collider, entity) in polyobject_query.query().iter() {
        let dist =
            collider.distance_to_point(global_transform, global_transform, world_cursor, false);
        info!("dist({entity}) = {dist}");
        if dist < min_dist && dist < SELECT_DISTANCE {
            closest = Some(entity)
        }
    }

    closest
}

pub fn select_closest_object(
    mut polyobject_query: QueryLens<(&GlobalTransform, &Collider, Entity)>,
    world_cursor: Vec2,
    mut selected_object: ResMut<SelectedObject>,
) {
    if let Some(entity) = find_closest_object(polyobject_query, world_cursor) {
        selected_object.select(entity)
    }
}