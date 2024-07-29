use bevy::prelude::Res;

use super::SelectedObject;

pub fn rc_object_selected(selected_object: Res<SelectedObject>) -> bool {
    selected_object.0.is_some()
}
