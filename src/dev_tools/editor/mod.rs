use avian2d::{
    math::AdjustPrecision,
    prelude::{Collider, IntoCollider, RigidBody},
};
use bevy::{
    color::palettes::tailwind,
    input::mouse::MouseButtonInput,
    math::{
        bounding::{Aabb2d, Bounded2d, BoundingCircle},
        vec2, vec3,
    },
    prelude::*,
    window::PrimaryWindow,
};

mod polyline;
use polyline::*;

// todo:
// [x] render controls with gizmos
// [ ] select objects
// [ ] spawn objects
// [ ] edit nodes
// [ ]
// [ ] box select
// [ ] generate colliders

pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SelectedObject>()
            .add_systems(Startup, test_setup)
            .add_systems(Update, (handle_mouse_input, draw_gizmos).chain());
    }
}

fn test_setup(mut commands: Commands) {
    let vertices = vec![vec2(0.0, 0.0), vec2(100.0, 100.0), vec2(160.0, 0.0)];
    let indices = vec![[0, 1], [1, 2], [2, 0]];

    let polyline = Polyline { vertices, indices };

    let collider = polyline.collider();

    let entity = commands
        .spawn((
            PolyObject {
                polyline: polyline.clone(),
                transform: Transform::from_translation(vec3(-300.0, 20000.0, 0.0)),
            },
            collider.clone(),
            RigidBody::Static,
        ))
        .id();

    commands.insert_resource(SelectedObject(Some(entity)));

    commands
        .spawn((
            PolyObject {
                polyline,
                transform: Transform::from_translation(vec3(-300.0, 20300.0, 0.0)),
            },
            collider,
            RigidBody::Static,
        ));
}

#[derive(Bundle)]
struct PolyObject {
    pub polyline: Polyline,
    pub transform: Transform,
}

impl PolyObject {
    pub fn new(origin: Vec2) -> Self {
        Self {
            polyline: Polyline::new(origin),
            transform: Transform::default(),
        }
    }

    pub fn translated(self, translation: Vec2) -> Self {
        let Self {
            polyline,
            mut transform,
        } = self;

        transform.translation = translation.extend(0.0);

        Self {
            transform,
            polyline,
        }
    }
}

#[derive(Resource, Default)]
struct SelectedObject(Option<Entity>);

fn handle_mouse_input(
    mut commands: Commands,
    selected_object: ResMut<SelectedObject>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    polyobject_query: Query<(&Polyline, &Transform)>,
) {
    if let Ok(window) = window_query.get_single() {
        let Some(cursor) = window.cursor_position() else {
            return;
        };
        
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // todo: search for existing objects/nodes

            if let Some(obj) = selected_object.0 {
                if let Ok((polyline, transform)) = polyobject_query.get(obj) {
                    // trick: transform mouse position, not the nodes
                    let local_cursor =
                        inverse_transform(*transform).transform_point(cursor.extend(0.0));

                    info!("{local_cursor}");

                    // Polyline2d
                }
            } else {
                // for (polyline, transform) in polyobject_query.iter()
            }

            // if there is an object selected then try to find it's node/edge

            // if node/edge is not found then try to find another object

            // if nothing is found do nothing
        }
    }
}

// todo: highligh selected
fn draw_gizmos(
    mut gizmos: Gizmos,
    polylines: Query<(&Polyline, &Transform, Entity)>,
    selected: Res<SelectedObject>,
) {
    for (polyline, transform, entity) in polylines.iter() {
        let positions: Vec<_> = polyline
            .vertices
            .iter()
            .map(|n| (*transform * n.extend(0.0)).truncate())
            .collect();

        let highlight = selected.0 == Some(entity);

        for pos in positions.iter() {
            gizmos.circle_2d(*pos, 5.0, tailwind::GREEN_300);
        }

        let color = if highlight {
            tailwind::AMBER_200
        } else {
            tailwind::GRAY_300
        };

        gizmos.linestrip_2d(positions, color);
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
