use avian2d::prelude::Collider;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::dev_tools::editor::{select_closest_object, Object, SelectedObject, SELECT_DISTANCE};

pub fn select_system(
    mut selected_object: ResMut<SelectedObject>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut object_query: Query<(&Object, &GlobalTransform, &Collider, Entity)>,
) {
    if let Ok(window) = window_query.get_single() {
        let Some(cursor) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        // this could be a separate system
        let Some(world_cursor) = camera
            .viewport_to_world(camera_transform, cursor)
            .map(|ray| {
                // info!("cursor world ray: {ray:?}"); // I'm curious what is the z coordinate of this, for me it's equal to 500.0...9
                ray.origin.truncate()
            })
        else {
            return;
        };

        if mouse_button_input.just_pressed(MouseButton::Left) {
            // todo: deselect & select on the same click?

            if let Some(obj) = selected_object.0 {
                if let Ok((polyline, global_transform, collider, _entity)) = object_query.get(obj) {
                    // trick: transform mouse position, not the nodes
                    // let local_cursor = inverse_transform(global_transform.compute_transform())
                    //     .transform_point(world_cursor.extend(0.0));

                    // info!("{local_cursor}");

                    let dist = collider.distance_to_point(
                        global_transform,
                        global_transform,
                        world_cursor,
                        false,
                    );
                    info!("dist to selected: {dist}");

                    if dist > SELECT_DISTANCE {
                        selected_object.deselect();
                    }
                    // Polyline2d
                }
            } else {
                select_closest_object(
                    object_query.transmute_lens::<(&GlobalTransform, &Collider, Entity)>(),
                    world_cursor,
                    selected_object,
                )
            }
        }
    }
}
