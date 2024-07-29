use bevy::{math::vec2, prelude::*};

use crate::dev_tools::editor::{Object, SelectedObject};

const MOVE_SPEED: f32 = 1.0;
const ROTATE_SPEED: f32 = 0.01;
const SCALE_SPEED: f32 = 0.01;

pub fn move_with_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut object_query: Query<(&mut Transform)>,
    selected_object: Res<SelectedObject>,
) {
    let Some(object) = selected_object.0 else {
        return;
    };

    let ctrl = input.pressed(KeyCode::ControlLeft);

    let mut motion = Vec2::default();
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyH]) {
        motion += vec2(-1.0, 0.0);
    }

    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyL]) {
        motion += vec2(1.0, 0.0);
    }

    if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyK]) {
        motion += vec2(0.0, 1.0);
    }

    if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyJ]) {
        motion += vec2(0.0, -1.0);
    }

    let Ok(mut transform) = object_query.get_mut(object) else {
        return;
    };

    if ctrl {
        transform.rotate_local_z(motion.x * ROTATE_SPEED);

        let delta_scale = 1.0 + motion.y * SCALE_SPEED;

        transform.scale *= delta_scale;
    } else {
        transform.translation += (motion * MOVE_SPEED).extend(0.0);
    }
}
