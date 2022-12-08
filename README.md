# olleth-simulation
Particle Simulation in rust

The particle simulation runs on bevy, a relativly new rust game engine. 

## Main function
To create a new program you need to create a app. to develop it you'll need to add resources, plugins that handles the underlying functions of your program. add_systems handles all functions that control what happens on the frontend for example, spawning in marbles and despawning them. I have also added a startup system which gets added before all other systems using .add_startup system. Dont forget to add the .run() in the end for the app to run!

### Example of how a main function looks like.
* XPBDPlugin: contains the physics behind the particles movement
* startup: Startup function
* spawn_marbles: Function that handles the spawning of particles
* despawn_marbles and despawn_marbles_at_height: handles how the particles despawn, either from user input or height. Exists to keep performance high.
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

## Startup function and particle creation

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

``` rust
//Example of how to spawn a rectangle into the scene
let size = Vec2::new(10., 2.);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::ONE))),
            material: blue.clone(),
            transform: Transform::from_scale(size.extend(1.)),
            ..Default::default()
        })
        .insert(StaticBoxBundle {
            pos: Pos(Vec2::new(0., -3.)),
            collider: BoxCollider { size },
            ..Default::default()
        });
```

``` rust
//Example of how to spawn in a particle
commands
    .spawn(PbrBundle {
        mesh: meshes.sphere.clone(),
        material: materials.blue.clone(),
        transform: Transform {
            scale: Vec3::splat(radius),
            translation: pos.extend(0.),
            ..Default::default()
        },
        ..Default::default()
            })
            .insert(ParticleBundle {
                collider: CircleCollider { radius },
                ..ParticleBundle::new_with_pos_and_vel(pos, vel)
            }); 
```
