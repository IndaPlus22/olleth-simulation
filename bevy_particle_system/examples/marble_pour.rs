use bevy::{time::FixedTimestep, prelude::*, input::mouse::{MouseMotion}};
use bevy_particle_system::*;
use rand::random;

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

#[derive(Resource)]
struct Materials {
    blue: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Meshes {
        sphere: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.,
            subdivisions: 4,
        })),
    });

    commands.insert_resource(Materials {
        blue: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.4, 0.6),
            unlit: true,
            ..Default::default()
        }),
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        projection: bevy::prelude::Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        }),
        ..Camera3dBundle::default()
    });

    let sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.,
        subdivisions: 4,
    }));

    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.6),
        unlit: true,
        ..Default::default()
    });

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

    commands.insert_resource(Meshes { sphere });
    commands.insert_resource(Materials { blue });
}

fn spawn_marbles(
    mut commands: Commands,
    materials: Res<Materials>, 
    meshes: Res<Meshes>, 
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>
) {
    let win = windows.get_primary().expect("no primary window");
    if buttons.pressed(MouseButton::Left) {

        let mouse_pos = win.cursor_position().unwrap();
        println!("{}, {}", mouse_pos.x / 100., mouse_pos.y / 100.);

        let radius = 0.1;
        let pos = Vec2::new(random::<f32>() - 0.5 + (mouse_pos.x / 100. - 8.5), random::<f32>() - 0.5 + (mouse_pos.y / 100. - 8.8)) * 0.5 + Vec2::Y * 3.;
        let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
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
    }
    
}

fn despawn_marbles_at_height(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20. {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>, buttons: Res<Input<MouseButton>>) {
    if buttons.pressed(MouseButton::Right) {
        for (entity, pos) in query.iter() {
            if pos.0.y > 1. { commands.entity(entity).despawn(); }
        }
    }   
}
