use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::spawn::{level::Ground, player::Player};

pub mod character_controller;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::new(FixedPostUpdate).with_length_unit(20.0)); //TODO: Needs to be adjusted probably
    app.add_plugins(character_controller::plugin);
    app.insert_resource(Gravity(Vector::NEG_Y * 1000.0));
    app.add_systems(FixedPreUpdate, update_gravity.in_set(AppSet::Update));
}

fn update_gravity(mut gravity: ResMut<Gravity>, query: Query<&Transform, With<Player>>) {
    if let Ok(t) = query.get_single() {
        gravity.0 = -t.translation.xy();
    }
}
