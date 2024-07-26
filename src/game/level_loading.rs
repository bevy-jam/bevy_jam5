use avian2d::prelude::{ColliderConstructor, RigidBody, VhacdParameters};
use bevy::{
    color::palettes::css::{PURPLE, WHITE},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    scene::SceneInstance,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SceneLoaded>();
    app.add_systems(Update, check_scene_loaded);
    app.observe(scene_to_2d_project);
}

// Define a custom event for our scene loading
#[derive(Event)]
struct SceneLoaded(Entity);

// Define a marker component to indicate what entities we've already processed
#[derive(Component)]
struct SceneProcessed;

// System to check if a scene has finished loading
fn check_scene_loaded(
    mut commands: Commands,
    query: Query<(Entity, &SceneInstance), Without<SceneProcessed>>,
    scene_spawner: Res<SceneSpawner>,
) {
    for (entity, instance) in query.iter() {
        if scene_spawner.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneProcessed);

            commands.trigger(SceneLoaded(entity));
        }
    }
}

fn scene_to_2d_project(
    trigger: Trigger<SceneLoaded>,
    children: Query<&Children>,
    query: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let entity = trigger.event().0;
    let mut entity_commands = commands.entity(entity);
    for child_entity in children.iter_descendants(entity) {
        if let Ok(handle) = query.get(child_entity) {
            let mesh = meshes.get(handle).unwrap();

            // Extract 3D vertices
            let vertices = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .as_float3()
                .unwrap();

            // Create 2D vertices (using X and Z components)
            let new_vertices: Vec<[f32; 3]> = vertices.iter().map(|v| [v[0], v[2], 0.0]).collect();
            let new_vertices_2d = new_vertices.iter().map(|n| Vec2::new(n[0], n[1])).collect();

            // Create a new 2D mesh
            let mut mesh2d = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            mesh2d.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);

            // If the original mesh has vertex normals, we need to adjust them for 2D
            if let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
                let new_normals: Vec<[f32; 3]> = normals
                    .as_float3()
                    .unwrap()
                    .iter()
                    .map(|n| [n[0], n[2], 0.0])
                    .collect();
                mesh2d.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
            }

            // If the original mesh has UV coordinates, we can keep them as is
            if let Some(uvs) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
                mesh2d.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.to_owned());
            }

            // If the original mesh has indices, we can keep them as is
            let Some(indices) = mesh.indices() else {
                continue;
            };
            let indices_2d = transform_indices(&indices);

            mesh2d = mesh2d.with_inserted_indices(indices.to_owned());

            // Add the new 2D mesh to the assets
            let mesh2d_handle = meshes.add(mesh2d);

            entity_commands.with_children(|parent| {
                parent.spawn((
                    RigidBody::Static,
                    ColliderConstructor::ConvexDecompositionWithConfig {
                        vertices: new_vertices_2d,
                        indices: indices_2d,
                        params: VhacdParameters {
                            concavity: 0.005,
                            alpha: 0.05,
                            beta: 0.05,
                            resolution: 256,
                            plane_downsampling: 24,
                            convex_hull_downsampling: 4,
                            fill_mode: avian2d::prelude::FillMode::SurfaceOnly,
                            convex_hull_approximation: false,
                            max_convex_hulls: 1024,
                        },
                    },
                    MaterialMesh2dBundle {
                        mesh: mesh2d_handle.into(),
                        material: materials.add(ColorMaterial {
                            color: PURPLE.into(),
                            ..Default::default()
                        }),
                        ..default()
                    },
                ));
            });
        }
    }
}

fn transform_indices(indices: &Indices) -> Vec<[u32; 2]> {
    // First, convert the indices to a Vec<u32> regardless of the original type
    let indices_u32: Vec<u32> = match indices {
        Indices::U16(vec) => vec.iter().map(|&i| i as u32).collect(),
        Indices::U32(vec) => vec.clone(),
    };

    // Now, transform the flat list into pairs
    // We're assuming that every two consecutive indices form a pair
    indices_u32
        .chunks(3) // Take chunks of 3 because we're converting from 3D to 2D
        .map(|chunk| {
            if chunk.len() >= 2 {
                [chunk[0], chunk[1]] // Take the first two indices of each triangle
            } else {
                // Handle the case where we don't have enough indices
                // This shouldn't happen if the input is valid, but we'll provide a default
                [0, 0]
            }
        })
        .collect()
}
