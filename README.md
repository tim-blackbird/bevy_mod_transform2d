# Bevy Mod Transform2d
[![Crates.io](https://img.shields.io/crates/v/bevy_mod_transform2d.svg)](https://crates.io/crates/bevy_mod_transform2d)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)

A 2D Transform component for [Bevy](https://github.com/bevyengine/bevy).

## Usage

Add the dependency to your `Cargo.toml`

```toml
[dependencies]
bevy_mod_transform2d = "0.3"
```

Example:

```rust
use bevy::prelude::*;
use bevy_mod_transform2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the Transform2dPlugin
        .add_plugin(Transform2dPlugin)
        .add_startup_system(setup)
        .add_system(orbit)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
        // Add a Transform2d component
        Transform2d::from_xy(200., 0.),
    ));
}

// Make the sprite orbit the center using the 2d transform.
fn orbit(mut query: Query<&mut Transform2d, With<Sprite>>, time: Res<Time>) {
    for mut transform in &mut query {
        // No quaternions in sight!
        transform.rotate_around(Vec2::ZERO, time.delta_seconds());
    }
}
```

Take a look at the other [examples](examples).

### Version table.

|Bevy  |transform2d
|-     |-
| 0.9  |0.3
| 0.8  |0.2
| 0.7  |0.1

## Further details

Note that the `Transform2d` component does not replace `Transform` component, instead it writes to it. The `Transform` and `GlobalTransform` components are required for `Transform2d` to function.

See [bevy_mod_2d_hierarchy] for an alternative plugin that does not require the `Transform` component.
Note that, for this reason, [bevy_mod_2d_hierarchy] is not compatible with crates that interact with `Transform` (eg. `bevy_rapier2d`),
and does not support parenting 3D transforms to 2D ones (useful for 2.5D games).

## Intergration with other crates

To integrate with another library that modifies `Transform` the state of `Transform` and `Transform2d` will need to be synchronised at the right times.

An intergration with `bevy_rapier2d` is included and can be enabled as a feature:
```toml
[dependencies]
bevy_mod_transform2d = { version = "0.3", features = ["bevy_rapier2d"] }
```

If there is another plugin that interacts with the transform here's how you would synchronise the state to make `Transform2d` compatible with that plugin.

* When a system needs to read from `Transfrom` add the provided `sync_transform_2d_to_3d` system *before* it.
* When a system writes to `Transfrom` add the provided `sync_transform_3d_to_2d` system *after* it.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[bevy_mod_2d_hierarchy]: https://github.com/ickshonpe/bevy_mod_2d_hierarchy