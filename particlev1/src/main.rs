#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use serde::Deserialize;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleSize {
    start: f32,
    end: f32,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleVelocity {
    start: Vec2,
    end: Vec2,
}

#[derive(Component, Clone, Copy, Deserialize)]
pub struct ParticleColor {
    start: Color,
    end: Color,
}

#[derive(Component)]
pub struct Particle {
    lifetime: Timer,
}

#[derive(Component)]
pub struct ParticleSpawnerTimer(Timer);

#[derive(Component, Deserialize)]
pub struct ParticleSpawner {
    rate: f32,
    amount_per_burst: usize,
    position_variance: f32,
    particle_lifetime: f32,
    particle_size: Option<ParticleSize>,
    particle_velocity: Option<ParticleVelocity>,
    particle_color: Option<ParticleColor>,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "I am a bevy window!".to_string(),
            width: 1000.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }))
    .insert_resource(WorldInspectorParams {
        enabled: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_particle_spawner)
    .add_system(update_particle_lifetime)
    .add_system(update_particle_size.after(emit_particles))
    .add_system(update_particle_position.after(emit_particles))
    .add_system(update_particle_color.after(emit_particles))
    .add_system(emit_particles)
    .run();
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::rgba(
        lerp(a.r(), b.r(), t),
        lerp(a.g(), b.g(), t),
        lerp(a.b(), b.b(), t),
        lerp(a.a(), b.a(), t),
    )
}

fn update_particle_lifetime(
    mut particles: Query<(&mut Particle, &mut Visibility)>,
    time: Res<Time>,
) {
    for (mut particle, mut visibility) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            visibility.is_visible = false;
        }
    }
}
fn update_particle_size(mut particles: Query<(&Particle, &ParticleSize, &mut Sprite)>) {
    for (particle, size, mut sprite) in particles.iter_mut() {
        let size = lerp(size.start, size.end, particle.lifetime.percent());
        sprite.custom_size = Some(Vec2::splat(size));
    }
}

fn update_particle_color(mut particles: Query<(&Particle, &ParticleColor, &mut Sprite)>) {
    for (particle, color, mut sprite) in particles.iter_mut() {
        let color = lerp_color(color.start, color.end, particle.lifetime.percent());
        sprite.color = color;
    }
}

fn update_particle_position(
    mut particles: Query<(&Particle, &ParticleVelocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (particle, velocity, mut transform) in particles.iter_mut() {
        let velocity = lerp_vec2(velocity.start, velocity.end, particle.lifetime.percent());
        transform.translation += (velocity * time.delta_seconds()).extend(0.0);
    }
}

fn spawn_particle(
    commands: &mut Commands,
    image: Option<Handle<Image>>,
    spawner: &ParticleSpawner,
) -> Entity {
    let mut sprite = Sprite::default();
    let image = match image {
        Some(image) => image,
        None => Handle::<Image>::default(),
    };
    let particle = commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(
                spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                000.0,
            ),
            texture: image,
            ..default()
        })
        .insert(Particle {
            lifetime: Timer::from_seconds(spawner.particle_lifetime, TimerMode::Once),
        })
        .insert(Sprite::default())
        .id();

        if let Some(size) = spawner.particle_size {
            sprite.custom_size = Some(Vec2::splat(size.start));
            commands.entity(particle).insert(size);
        }
        if let Some(color) = spawner.particle_color {
            sprite.color = color.start;
            commands.entity(particle).insert(color);
        }
        if let Some(velocity) = spawner.particle_velocity {
            commands.entity(particle).insert(velocity);
        }
        commands.entity(particle).insert(sprite);
        particle
}



fn emit_particles(
    mut spawners: Query<(&Children, &ParticleSpawner, &mut ParticleSpawnerTimer)>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    time: Res<Time>,
) {
    for (children, spawner, mut timer) in spawners.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            for _i in 0..spawner.amount_per_burst {
                for child in children.iter() {
                    if let Ok((mut particle, mut visibility, mut transform)) =
                        particles.get_mut(*child)
                    {
                        if !visibility.is_visible {
                            particle.lifetime =
                                Timer::from_seconds(spawner.particle_lifetime, TimerMode::Once);
                            visibility.is_visible = true;
                            transform.translation = Vec3::new(
                                spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                                spawner.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                                0.0,
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn spawn_particle_spawner(mut commands: Commands ,assets: Res<AssetServer>) {

    let ron_str = &std::fs::read_to_string("assets/basic_spawner.ron").unwrap();
    let spawner =
        ron::from_str::<ParticleSpawner>(ron_str).expect("Failed to load basic_spawner.ron");

    let mut particles = Vec::new();
    for _i in 0..((1.1 * spawner.particle_lifetime / spawner.rate).ceil() as usize
        * spawner.amount_per_burst)
    {
        particles.push(spawn_particle(&mut commands, Some(assets.load("leaf.png")), &spawner));
    }

    commands
        .spawn(SpatialBundle::default())
        .insert(ParticleSpawnerTimer(Timer::from_seconds(
            spawner.rate,
            TimerMode::Repeating,
        )))
        .insert(spawner)
        .push_children(&particles);

    let ron_str = &std::fs::read_to_string("assets/basic_spawner2.ron").unwrap();
    let spawner =
        ron::from_str::<ParticleSpawner>(ron_str).expect("Failed to load basic_spawner.ron");

    let mut particles = Vec::new();
    for _i in 0..((1.1 * spawner.particle_lifetime / spawner.rate).ceil() as usize
        * spawner.amount_per_burst)
    {
        particles.push(spawn_particle(&mut commands, Some(assets.load("leaf.png")), &spawner));
    }

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            1.0, 1.5, 0.0,
        )))
        .insert(ParticleSpawnerTimer(Timer::from_seconds(
            spawner.rate,
            TimerMode::Repeating,
        )))
        .insert(spawner)
        .push_children(&particles);
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}