//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{color::palettes::tailwind::GRAY_600, dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use editor::{Editor, EditorState};

use bevy_egui::{egui, EguiContext, EguiSettings};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::{bevy_inspector, prelude::*, reflect_inspector};

use crate::screen::Screen;

mod editor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EguiPlugin,
        DefaultInspectorConfigPlugin,
        PhysicsDebugPlugin::default(),
        Editor,
    ))
    .insert_resource(EguiSettings {
        scale_factor: 1.5,
        ..default()
    });

    // let physics_gizmos = PhysicsGizmos::default().with_aabb_color(GRAY_600.with_alpha(0.7).into());
    // app.insert_gizmo_config(physics_gizmos, GizmoConfig::default());

    app.add_systems(
        Update,
        (
            log_transitions::<Screen>,
            log_transitions::<EditorState>,
            inspector_ui,
        ),
    );
}

fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::TopBottomPanel::bottom("Status Bar").show(egui_context.get_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Editor status: ");
            bevy_inspector::ui_for_resource::<State<EditorState>>(world, ui);
        });

    });
}
