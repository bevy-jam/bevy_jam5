use bevy::{
    color::palettes::css::{PURPLE, WHITE},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
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

            println!("2D vertices: {:?}", new_vertices);

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
            if let Some(indices) = mesh.indices() {
                mesh2d = mesh2d.with_inserted_indices(indices.to_owned());
            }

            // Add the new 2D mesh to the assets
            let mesh2d_handle = meshes.add(mesh2d);

            entity_commands.with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: mesh2d_handle.into(),
                    transform: Transform::from_scale(Vec3::splat(5.0)),
                    material: materials.add(ColorMaterial {
                        color: PURPLE.into(),
                        ..Default::default()
                    }),
                    ..default()
                });
            });
        }
    }
}
