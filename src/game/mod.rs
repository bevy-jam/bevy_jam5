//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod character_controller;
mod movement;
mod physics;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        character_controller::plugin,
        movement::plugin,
        spawn::plugin,
        physics::plugin,
    ));
}
