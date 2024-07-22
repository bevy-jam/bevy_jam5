//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use avian2d::prelude::{Gravity, LinearVelocity};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::spawn::player::Player;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Apply movement based on controls.
    app.register_type::<WrapWithinWindow>();
    app.add_systems(
        FixedUpdate,
        (wrap_within_window).chain().in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WrapWithinWindow;

fn wrap_within_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<WrapWithinWindow>>,
) {
    let size = window_query.single().size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
