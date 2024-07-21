use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screen::Screen;

use super::spawn::level::Ground;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::new(FixedPostUpdate).with_length_unit(20.0)); //TODO: Needs to be adjusted probably
    app.register_type::<GravityController>();

    app.add_systems(Update, apply_gravity.run_if(in_state(Screen::Playing)));
}

#[derive(Default, Debug, Component, Reflect)]
pub struct GravityController(pub f32);

/// Applies [`ControllerGravity`] to character controllers.
fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&Transform, &mut LinearVelocity, &GravityController)>,
    ground: Query<&Transform, With<Ground>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds();
    let origin = ground.single(); //TODO: Or whatever the origin point is

    for (transform, mut linear_velocity, gravity_controller) in &mut controllers {
        let gravity = origin.translation.truncate() - transform.translation.truncate();
        println!("{:?}", gravity);
        linear_velocity.0 += gravity.normalize() * gravity_controller.0 * delta_time;
        println!("{:?}", linear_velocity);
    }
}
