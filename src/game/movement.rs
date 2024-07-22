//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use avian2d::prelude::{Gravity, LinearVelocity};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::spawn::player::{Player, PlayerSpeed};
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.add_systems(
        FixedUpdate,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.add_systems(FixedPreUpdate, update_gravity.in_set(AppSet::Update));
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&mut LinearVelocity, &PlayerSpeed, &Transform)>,
) {
    // Collect directional input.
    let mut horizontal = 0.;
    // TODO: This does not really work because movement needs to depend on the "orientation" of the player
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        horizontal -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }

    let mut jump = false;

    if input.just_pressed(KeyCode::Space) {
        jump = true;
    }

    // Apply movement intent to controllers.
    for (mut velocity, speed, transform) in &mut controller_query {
        let normal = -transform.translation.xy().perp().normalize();
        velocity.0 += normal * horizontal * speed.0; //TODO: This does not really work, because we never take away the velocity (e.g. collision, damping)
        if jump {
            velocity.0 += transform.translation.xy().normalize() * 500.;
        }
    }
}

fn update_gravity(mut gravity: ResMut<Gravity>, query: Query<&Transform, With<Player>>) {
    if let Ok(t) = query.get_single() {
        gravity.0 = -t.translation.xy();
    }
}
