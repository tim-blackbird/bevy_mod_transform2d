use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // Add the Transform2dPlugin after the RapierPhysicsPlugin.
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Boxes
    for x in -3..4 {
        for y in -2..4 {
            commands.spawn((
                Transform2dBundle::from(Transform2d::from_xy(x as f32 * 100., y as f32 * 100.)),
                RigidBody::Dynamic,
                Collider::cuboid(45., 45.),
            ));
        }
    }

    // Platform
    commands.spawn((
        Transform2dBundle::from(Transform2d::from_xy(0., -300.)),
        RigidBody::Fixed,
        Collider::cuboid(400., 20.),
    ));

    // Spawn camera
    commands.spawn(Camera2dBundle::default());
}
