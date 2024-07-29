use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};

use crate::dev_tools::editor::{PolyBundle, Polyline};

use super::SelectedObject;

pub fn test_setup(mut commands: Commands) {
    let vertices = vec![vec2(0.0, 0.0), vec2(100.0, 100.0), vec2(160.0, 0.0)];
    let indices = vec![[0, 1], [1, 2], [2, 0]];

    let polyline = Polyline { vertices, indices };

    let collider = polyline.collider();

    let entity = commands
        .spawn((
            PolyBundle::new(polyline.clone()).translated(vec2(-300.0, 20000.0)),
            collider.clone(),
            RigidBody::Static,
            DebugRender::default().without_axes()
        ))
        .id();

    commands.insert_resource(SelectedObject(Some(entity)));

    commands.spawn((
        PolyBundle::new(polyline.clone()).translated(vec2(-300.0, 20200.0)),
        collider,
        RigidBody::Static,
        DebugRender::default().without_axes()
    ));
}
