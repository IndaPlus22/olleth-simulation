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

```
