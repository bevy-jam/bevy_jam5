use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0)); //TODO: Needs to be adjusted probably
    app.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 5000.0));
}
