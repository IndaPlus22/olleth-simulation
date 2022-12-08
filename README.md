# olleth-simulation
The particle simulation runs on bevy, a relativly new rust game engine. The example folder contains the different stages of completetion. The src folder handles the physics behind the particles when they are spawned in.

### Example Folder
* simple.rs - Simplest example of how bevy operates.
* particle_collisions.rs - checks that the collision physics is operating correctly.
* different_masses.rs - checks if the physics behind particles with the different masses works arcordingly.
* marble_pour.rs - A simple testing ground for particle system.
* ball_stacking.rs - Work in Progress...

## Main function
To create a new program you need to create a app. to develop it you'll need to add resources, plugins that handles the underlying functions of your program. add_systems handles all functions that control what happens on the frontend for example, spawning in marbles and despawning them. I have also added a startup system which gets added before all other systems using .add_startup system. Dont forget to add the .run() in the end for the app to run!

### Example of how a main function looks like
* XPBDPlugin: contains the physics behind the particles movement.
* startup: Startup function.
* spawn_marbles: Function that handles the spawning of particles.
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
To create particles we need to create materials and meshes for them. We do that by using structs.
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
## Mesh and Material creation
We then need to create the meshes and materials as you see down below. For the app to recognise the meshes and material we need to insert them into the apps commands.
``` rust
//Creates a resource for the circle meshes
commands.insert_resource(Meshes {
        sphere: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1., //Radius of the circle
            subdivisions: 4, //Devides the circle into n areas. More subdivisions mean rounder circle
        })),
    });
 
 //Creates a resource for the color meshes
 commands.insert_resource(Materials {
        blue: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.4, 0.6), //Materials color
            unlit: true, //Bool value that checks whether to apply only the base color to this material.
            ..Default::default()
        }),
    });

```
 ## Creating and spawning a camera
 To see what happens on the screen when we run the program we need a camera. We'll use this simple camera3dBundle that can be found inm bevys cheatcode Book.
 After creating the bundle we need to spawn it in using commands.spawn()
``` rust
//Creates a 3d camera and adds it to the scene
commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)), //Where to position the camera
        projection: bevy::prelude::Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        }),
        ..Camera3dBundle::default()
    });
```

 ## Spawning rectangles for collision testing
 To check whether the particles can collide with other surfaces we need to spawn in other objects such as a rectangle. The code below shows a simple way to do just that.
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
## Spawning the particles
Looks similar to the rectangle bundle with a few changes to handle the position and velocity of the particles when spawned in. ParticleBundle contains all variables need for the particle physics (Code inside of src/entity.rs).
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
