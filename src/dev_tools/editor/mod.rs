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
// [ ] hower
// [ ] display mode on screen
// [x] select objects
// [ ] multi select
// [ ] spawn objects
// [x] move objects
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
            .add_systems(Startup, test_setup)
            .add_systems(Update, mode_switch_system)
            .add_systems(
                Update,
                (
                    select_system.run_if(in_state(EditorState::SelectMode)),
                    move_with_keys.run_if(in_state(EditorState::MoveMode).and_then(rc_object_selected)),
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

#[derive(Component, Reflect)]
pub enum Object {
    Polyline,
    Sprite,
}

#[derive(Resource, Default, Reflect)]
pub struct SelectedObject(Option<Entity>);

impl SelectedObject {
    pub fn select(&mut self, entity: Entity) {
        self.0 = Some(entity)
    }
    pub fn deselect(&mut self) {
        self.0 = None
    }
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
