//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::*;
use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use polyeditor::PolyEditor;

use crate::screen::Screen;

mod polyeditor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::new(),
        PhysicsDebugPlugin::default(),
        PolyEditor,
    ));
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
}
