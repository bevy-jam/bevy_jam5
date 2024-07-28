use avian2d::{
    math::AdjustPrecision,
    prelude::{Collider, ColliderAabb, DebugRender, IntoCollider, RigidBody},
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
// [ ] hower
// [x] select objects
// [ ] spawn objects
// [ ] edit nodes
// [ ]
// [ ] box select
// [ ] generate colliders

const SELECT_DISTANCE: f32 = 12.0;

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
                transform: Transform::from_translation(vec3(-300.0, 20000.0, 0.0)).into(),
            },
            collider.clone(),
            RigidBody::Static,
        ))
        .id();

    commands.insert_resource(SelectedObject(Some(entity)));

    commands.spawn((
        PolyObject {
            polyline,
            transform: Transform::from_translation(vec3(-300.0, 20300.0, 0.0)).into(),
        },
        collider,
        RigidBody::Static,
    ));
}

#[derive(Bundle)]
struct PolyObject {
    pub polyline: Polyline,
    pub transform: TransformBundle,
}

impl PolyObject {
    pub fn new(origin: Vec2) -> Self {
        Self {
            polyline: Polyline::new(origin),
            transform: TransformBundle::default(),
        }
    }

    pub fn translated(self, translation: Vec2) -> Self {
        let Self {
            polyline,
            mut transform,
        } = self;

        transform.local.translation = translation.extend(0.0);

        Self {
            transform,
            polyline,
        }
    }
}

#[derive(Resource, Default)]
struct SelectedObject(Option<Entity>);

impl SelectedObject {
    pub fn select(&mut self, entity: Entity) {
        self.0 = Some(entity)
    }
    pub fn deselect(&mut self) {
        self.0 = None
    }
}

fn handle_mouse_input(
    mut commands: Commands,
    mut selected_object: ResMut<SelectedObject>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    polyobject_query: Query<(&Polyline, &GlobalTransform, &Collider, Entity)>,
) {
    if let Ok(window) = window_query.get_single() {
        let Some(cursor) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        let Some(world_cursor) = camera
            .viewport_to_world(camera_transform, cursor)
            .map(|ray| {
                // info!("cursor world ray: {ray:?}"); // I'm curious what is the z coordinate of this, for me it's equal to 500.0...9
                ray.origin.truncate()
            })
        else {
            return;
        };

        if mouse_button_input.just_pressed(MouseButton::Left) {
            // todo: deselect & select on the same click?

            if let Some(obj) = selected_object.0 {
                if let Ok((polyline, global_transform, collider, _entity)) =
                    polyobject_query.get(obj)
                {
                    // trick: transform mouse position, not the nodes
                    // let local_cursor = inverse_transform(global_transform.compute_transform())
                    //     .transform_point(world_cursor.extend(0.0));

                    // info!("{local_cursor}");

                    let dist = collider.distance_to_point(
                        global_transform,
                        global_transform,
                        world_cursor,
                        false,
                    );
                    info!("dist to selected: {dist}");

                    if dist > SELECT_DISTANCE {
                        selected_object.deselect();
                    }
                    // Polyline2d
                }
            } else {
                let mut min_dist = f32::MAX;
                let mut closest = None;

                // todo: optimize with aabb?
                for (polyline, global_transform, collider, entity) in polyobject_query.iter() {
                    let dist = collider.distance_to_point(
                        global_transform,
                        global_transform,
                        world_cursor,
                        false,
                    );
                    info!("dist({entity}) = {dist}");
                    if dist < min_dist && dist < SELECT_DISTANCE {
                        closest = Some(entity)
                    }
                }

                if let Some(entity) = closest {
                    selected_object.select(entity)
                }
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
    collider_aabbs: Query<&ColliderAabb>,
    selected: Res<SelectedObject>,
) {
    for (polyline, transform, entity) in polylines.iter() {
        let positions: Vec<_> = polyline
            .vertices
            .iter()
            .map(|n| (*transform * n.extend(0.0)).truncate())
            .collect();

        let highlight = selected.0 == Some(entity);

        let color = if highlight {
            tailwind::GREEN_200
        } else {
            tailwind::GRAY_300
        };

        for pos in positions.iter() {
            gizmos.circle_2d(*pos, 5.0, color);
        }

        let color = if highlight {
            tailwind::AMBER_200
        } else {
            tailwind::GRAY_300
        };

        gizmos.linestrip_2d(positions, color);
    }

    for (collider_aabb) in collider_aabbs.iter() {

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
