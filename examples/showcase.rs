use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .add_system(orbit)
        .add_system(rotate)
        .run();
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
    // Spawn a sprite with a 2d transform at the center.
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

    // Spawn a sprite with a 2d transform that orbits the center.
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

fn orbit(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform2d)>) {
    for (orbit, mut transform) in &mut query {
        transform.translate_around(orbit.point, orbit.speed * time.delta_seconds());
    }
}

fn rotate(time: Res<Time>, mut query: Query<(&Rotate, &mut Transform2d)>) {
    for (rotate, mut transform) in &mut query {
        transform.rotation += rotate.speed * time.delta_seconds();
    }
}
