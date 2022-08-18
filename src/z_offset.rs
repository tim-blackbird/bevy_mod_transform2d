use bevy::{prelude::*, reflect::FromReflect};

/// An optional component used to offset the render order for sprites.
///
/// This component maps directly to the z translation of the Transform,
/// and if not present the z translation is left as it was.
///
/// If this [`Transform2d`](crate::Transform2d) has a parent, the offset is relative to the [`Transform2d`] or [`Transform`] of the parent.
#[derive(Component, Debug, PartialEq, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component, PartialEq, Default)]
pub struct ZOffset(pub f32);

impl From<f32> for ZOffset {
    fn from(z: f32) -> Self {
        ZOffset(z)
    }
}

impl From<ZOffset> for f32 {
    fn from(z_offset: ZOffset) -> Self {
        z_offset.0
    }
}
