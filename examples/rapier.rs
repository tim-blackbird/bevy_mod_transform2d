use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Transform2dPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(100.),
            RapierDebugRenderPlugin::default(),
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
        RigidBody::KinematicPositionBased,
        Collider::cuboid(400., 20.),
    ));

    // Boxes
    for x in -3..4 {
        for y in -2..4 {
            commands.spawn((
                Transform2dBundle::from(Transform2d::from_xy(x as f32 * 100., y as f32 * 100.)),
                RigidBody::Dynamic,
                Collider::cuboid(50., 50.),
            ));
        }
    }

    commands.spawn(Camera2dBundle::default());
}
