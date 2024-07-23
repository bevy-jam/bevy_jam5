use avian2d::prelude::*;
use bevy::{
    color::palettes::{css::GREEN, tailwind::BLUE_300},
    input::mouse::MouseMotion,
    prelude::*,
};

use crate::{screen::Screen, AppSet};

use super::spawn::{level::Ground, player::Player};

const GRAVITY: f32 = 1000.0;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::new(FixedPostUpdate).with_length_unit(20.0)); //TODO: Needs to be adjusted probably
    app.register_type::<GravityController>();

    // app.add_systems(Update, apply_gravity.run_if(in_state(Screen::Playing)));
    app.add_systems(
        FixedPreUpdate,
        update_gravity
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Default, Debug, Component, Reflect)]
pub struct GravityController(pub f32);

fn update_gravity(
    mut gravity: ResMut<Gravity>,
    query: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
) {
    if let Ok(t) = query.get_single() {
        let player_pos = t.translation.xy();

        gravity.0 = -player_pos.normalize_or_zero() * GRAVITY;
        gizmos.arrow_2d(
            player_pos,
            player_pos + gravity.0 / GRAVITY * 20.0,
            BLUE_300,
        );
    }
}
