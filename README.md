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
