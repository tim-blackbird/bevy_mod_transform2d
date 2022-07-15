use bevy::prelude::*;

/// An optional component used to set the render order for sprites.
/// This component maps directly to the z translation of the Transform,
/// and if not present the z translation is left as it was.
#[derive(Component, Debug, PartialEq, Clone, Copy, Default, Reflect)]
#[reflect(Component, PartialEq)]
pub struct ZIndex(pub f32);
