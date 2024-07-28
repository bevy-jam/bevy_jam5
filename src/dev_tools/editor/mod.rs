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

mod run_conditions;
use run_conditions::*;

// todo:
// [x] render controls with gizmos
// [ ] hower
// [x] select objects
// [x] multi select
// [ ] spawn objects
// [ ] move objects
// [ ] edit nodes
// [ ]
// [ ] box select
// [ ] generate colliders

const SELECT_DISTANCE: f32 = 12.0;

pub struct Editor;

impl Plugin for Editor {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SelectedObject>()
            .init_resource::<EditMode>()
            .add_systems(Startup, test_setup)
            .add_systems(
                Update,
                (select_system.run_if(rc_select_mode), draw_gizmos).chain(),
            );
    }
}

fn test_setup(mut commands: Commands) {
    let vertices = vec![vec2(0.0, 0.0), vec2(100.0, 100.0), vec2(160.0, 0.0)];
    let indices = vec![[0, 1], [1, 2], [2, 0]];

    let polyline = Polyline { vertices, indices };

    let collider = polyline.collider();

    let entity = commands
        .spawn((
            PolyBundle::new(polyline.clone()).translated(vec2(-300.0, 20000.0)),
            collider.clone(),
            RigidBody::Static,
        ))
        .id();

    commands.insert_resource(SelectedObject(Some(entity)));

    commands.spawn((
        PolyBundle::new(polyline.clone()).translated(vec2(-300.0, 20200.0)),
        collider,
        RigidBody::Static,
    ));
}

#[derive(Component, Reflect)]
enum Object {
    Polyline,
    Sprite,
}

#[derive(Resource, Reflect, Default)]
enum EditMode {
    #[default]
    Select,
    Move,
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

fn select_system(
    mut selected_object: ResMut<SelectedObject>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut object_query: Query<(&Object, &GlobalTransform, &Collider, Entity)>,
) {
    if let Ok(window) = window_query.get_single() {
        let Some(cursor) = window.cursor_position() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.get_single() else {
            return;
        };

        // this could be a separate system
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
                if let Ok((polyline, global_transform, collider, _entity)) = object_query.get(obj) {
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
                select_closest_object(
                    object_query.transmute_lens::<(&GlobalTransform, &Collider, Entity)>(),
                    world_cursor,
                    selected_object,
                )
            }
        }
    }
}


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

    for (collider_aabb) in collider_aabbs.iter() {}
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
