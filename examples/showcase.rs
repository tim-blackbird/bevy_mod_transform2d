use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;

fn main() {
    App::new()
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
struct Rotator {
    speed: f32,
}

fn setup(mut commands: Commands) {
    // Spawn a sprite with a 2d transform at the center.
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((Transform2d::default(), Rotator { speed: 1. }));

    // Spawn a sprite with a 2d transform that orbits the center.
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            Transform2d::from_xy(200., 0.),
            Orbit {
                point: Vec2::ZERO,
                speed: 1.5,
            },
            Rotator { speed: -1.2 },
        ));

    // Spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn orbit(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform2d)>) {
    let delta = time.delta_seconds();
    for (orbit, mut transform) in query.iter_mut() {
        transform.translate_around(orbit.point, orbit.speed * delta);
    }
}

fn rotate(time: Res<Time>, mut query: Query<(&Rotator, &mut Transform2d)>) {
    let delta = time.delta_seconds();
    for (rotator, mut transform) in &mut query.iter_mut() {
        transform.rotation += rotator.speed * delta;
    }
}
