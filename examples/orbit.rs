use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, Transform2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit, rotate))
        .run();
}

fn orbit(mut query: Query<(&mut Transform2d, &Orbit)>, time: Res<Time>) {
    for (mut transform, orbit) in &mut query {
        transform.translate_around(orbit.point, orbit.speed * time.delta_seconds());
    }
}

fn rotate(mut query: Query<(&mut Transform2d, &Rotate)>, time: Res<Time>) {
    for (mut transform, rotate) in &mut query {
        transform.rotation += rotate.speed * time.delta_seconds();
    }
}

#[derive(Component)]
struct Orbit {
    point: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Rotate {
    speed: f32,
}

fn setup(mut commands: Commands) {
    // Spawn a sprite in the center with a 2d transform.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::CRIMSON,
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
        Transform2d::default(),
        Rotate { speed: 1. },
    ));

    // Spawn a sprite that orbits the center with a 2d transform.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                custom_size: Some(Vec2::splat(150.)),
                ..default()
            },
            ..default()
        },
        Transform2d::from_xy(200., 0.),
        Orbit {
            point: Vec2::ZERO,
            speed: 1.5,
        },
        Rotate { speed: -1.2 },
    ));

    // Spawn the camera.
    commands.spawn(Camera2dBundle::default());
}
