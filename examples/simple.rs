use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..default()
        })
        // Transform2d will not function without a Transform and GlobalTransform present.
        // Here they are provided by the SpriteBundle above.
        .insert(Transform2d::identity())
        // A separate optional component can be used to change the render order for sprites.
        // This component maps directly to the z translation of the Transform and if this component is not present.
        .insert(ZIndex(1.));
    
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::BLACK,
                ..default()
            },
            ..default()
        })
        // An entity without a ZIndex component will 
        .insert(Transform2d::from_xy(50., 0.));

    // Spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
