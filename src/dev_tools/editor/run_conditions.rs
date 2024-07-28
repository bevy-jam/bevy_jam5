use bevy::prelude::Res;

use super::{EditMode, SelectedObject};

pub fn rc_object_selected(selected_object: Res<SelectedObject>) -> bool {
    selected_object.0.is_some()
}

pub fn rc_select_mode(mode: Res<EditMode>) -> bool {
    match *mode {
        EditMode::Select => true,
        _ => false,
    }
}

pub fn rc_move_mode(mode: Res<EditMode>) -> bool {
    match *mode {
        EditMode::Move => true,
        _ => false,
    }
}
