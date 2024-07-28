use avian2d::{
    math::AdjustPrecision,
    prelude::{Collider, ColliderAabb, DebugRender, IntoCollider, RigidBody},
};
use bevy::{
    color::palettes::tailwind,
    ecs::system::QueryLens,
    input::mouse::MouseButtonInput,
    math::{
        bounding::{Aabb2d, Bounded2d, BoundingCircle},
        vec2, vec3,
    },
    prelude::*,
    window::PrimaryWindow,
};

mod util;
use util::*;

mod polyline;
use polyline::*;

mod systems;
pub use systems::*;

mod run_conditions;
use run_conditions::*;

// todo:
// [x] render controls with gizmos
// [x] hover
// [x] display mode on screen
// [x] select objects
// [ ] multi select
// [ ] spawn objects
// [ ] copy paste
// [x] move objects
// [x] rotate objects
// [x] scale objects
// [ ] edit nodes
// [ ] undo
// [ ]
// [ ] box select
// [ ] generate colliders

const SELECT_DISTANCE: f32 = 12.0;

pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<EditorState>();

        app.configure_sets(
            Update,
            (EditorSet::HandleInput, EditorSet::Apply, EditorSet::Display).chain(),
        );

        app.init_resource::<SelectedObject>()
            .init_resource::<HoveredObject>()
            .init_resource::<WorldCursor>()
            .add_systems(Startup, test_setup)
            .add_systems(
                Update,
                (
                    update_world_cursor.before(InputKind::Mouse),
                    mode_switch_system,
                    cursor_object_select_system
                        .run_if(in_state(EditorState::SelectMode))
                        .in_set(InputKind::Mouse),
                    move_with_keys
                        .run_if(in_state(EditorState::MoveMode).and_then(rc_object_selected))
                        .in_set(InputKind::Keyboard),
                )
                    .in_set(EditorSet::HandleInput),
            )
            .add_systems(Update, (draw_gizmos,).in_set(EditorSet::Display));
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum EditorSet {
    HandleInput,
    Apply,
    Display,
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum InputKind {
    Mouse,
    Keyboard,
}

#[derive(Component, Reflect)]
pub enum Object {
    Polyline,
    Sprite,
}

// todo: move to helpers
fn inverse_transform(transform: Transform) -> Transform {
    let Transform {
        translation,
        rotation,
        scale,
    } = transform;
    Transform {
        translation: -translation,
        rotation: rotation.inverse(),
        scale: scale.recip(),
    }
}
