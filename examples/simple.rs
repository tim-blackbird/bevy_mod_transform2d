use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // Be sure to add Transform2dPlugin after the RapierPhysicsPlugin.
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a SpriteBundle that has the Transform and GlobalTransform components.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..default()
        },
        Transform2d::IDENTITY,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::BLACK,
                ..default()
            },
            ..default()
        },
        Transform2d::from_xy(50., 50.),
    ));

    // Spawn camera
    commands.spawn(Camera2dBundle::default());
}
