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
    commands // Spawn a SpriteBundle that has the Transform and GlobalTransform components.
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..default()
        })
        .insert_bundle(Transform2dBundle::identity());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::BLACK,
                ..default()
            },
            ..default()
        })
        .insert_bundle(Transform2dBundle::from_transform(Transform2d::from_xy(
            50., 0.,
        )));

    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());
}
