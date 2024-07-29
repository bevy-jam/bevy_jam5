use avian2d::prelude::Collider;
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::dev_tools::editor::{Object, WorldCursor, SELECT_DISTANCE};

#[derive(Resource, Default, Reflect)]
pub struct SelectedObject(pub Option<Entity>);

impl SelectedObject {
    pub fn select(&mut self, entity: Entity) {
        self.0 = Some(entity)
    }
    pub fn deselect(&mut self) {
        self.0 = None
    }
}

#[derive(Resource, Default, Reflect)]
pub struct HoveredObject(pub Option<Entity>);

impl HoveredObject {
    pub fn hover(&mut self, entity: Entity) {
        self.0 = Some(entity)
    }

    pub fn clean(&mut self) {
        self.0 = None
    }
}

pub fn cursor_object_select_system(
    mut hovered_object: ResMut<HoveredObject>,
    mut selected_object: ResMut<SelectedObject>,
    mut mouse_motion: EventReader<MouseMotion>,
    world_cursor: Res<WorldCursor>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut object_query: Query<(&Object, &GlobalTransform, &Collider, Entity)>,
) {
    let left_mouse_pressed = mouse_button_input.just_pressed(MouseButton::Left);
    let cursor_moved = mouse_motion.read().count() > 0;

    if !cursor_moved && !left_mouse_pressed {
        return;
    }

    hovered_object.clean();

    let Some(world_cursor) = world_cursor.0 else {
        return;
    };

    let mut min_dist = f32::MAX;
    let mut closest = None;

    // todo: optimize with aabb?
    for (object, global_transform, collider, entity) in object_query.iter() {
        let dist =
            collider.distance_to_point(global_transform, global_transform, world_cursor, false);
        // info!("dist({entity}) = {dist}");
        if dist < min_dist && dist < SELECT_DISTANCE {
            closest = Some(entity)
        }
    }

    if left_mouse_pressed {
        if let Some(entity) = closest {
            selected_object.select(entity);
        } else {
            selected_object.deselect();
        }
    } else {
        hovered_object.0 = closest
    }
}
