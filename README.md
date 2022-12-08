# olleth-simulation
Particle Simulation in rust

``` rust

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(XPBDPlugin::default())
        .add_plugin(bevy_editor_pls::EditorPlugin)
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 20.))
                .with_system(spawn_marbles),
        )
        .add_system(despawn_marbles)
        .add_system(despawn_marbles_at_height)
        .run();
}

```

``` rust

//Simple struct to handle the  particles material
#[derive(Resource)]
struct Materials {
    blue: Handle<StandardMaterial>,
}

//Simple struct to handle the particle meshes
#[derive(Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

```

``` rust
//Creates a resource for the circle meshes
commands.insert_resource(Meshes {
        sphere: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.,
            subdivisions: 4,
        })),
    });
 
 //Creates a resource for the color meshes
 commands.insert_resource(Materials {
        blue: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.4, 0.6),
            unlit: true,
            ..Default::default()
        }),
    });

```
 
``` rust
//Creates a 3d camera and adds it to the scene
commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        projection: bevy::prelude::Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        }),
        ..Camera3dBundle::default()
    });
```
