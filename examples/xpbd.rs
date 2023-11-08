use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

fn main() {
    App::new()
        .insert_resource(Gravity(Vector::NEG_Y * 1000.))
        .add_plugins((
            DefaultPlugins,
            Transform2dPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_platform)
        .run();
}

#[derive(Component)]
struct Platform;

fn rotate_platform(mut platform: Query<&mut Transform2d, With<Platform>>, time: Res<Time>) {
    for mut transform in &mut platform {
        transform.rotation = time.elapsed_seconds().cos() * 0.1;
    }
}

fn setup(mut commands: Commands) {
    // Platform
    commands.spawn((
        Platform,
        Transform2dBundle::from(Transform2d::from_xy(0., -320.)),
        RigidBody::Kinematic,
        Collider::cuboid(800., 40.),
    ));

    // Boxes
    for x in -3..4 {
        for y in -2..4 {
            commands.spawn((
                Transform2dBundle::from(Transform2d::from_xy(x as f32 * 100., y as f32 * 100.)),
                RigidBody::Dynamic,
                Collider::cuboid(99., 99.),
            ));
        }
    }

    commands.spawn(Camera2dBundle::default());
}
