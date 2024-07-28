//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::*;
use bevy::{color::palettes::tailwind::GRAY_600, dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use editor::Editor;

use crate::screen::Screen;

mod editor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::new(),
        PhysicsDebugPlugin::default(),
        Editor,
    ));

    let group = PhysicsGizmos::default().with_aabb_color(GRAY_600.with_alpha(0.7).into());
    app.insert_gizmo_config(group, GizmoConfig::default());

    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
}
