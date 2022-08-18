use bevy::prelude::*;

use crate::{transform_2d::Transform2d, z_offset::ZOffset};

#[derive(Bundle, Clone, Copy, Debug, Default, Reflect)]
pub struct Transform2dBundle {
    pub transform: Transform2d,
    pub z_offset: ZOffset,
    pub transform_3d: Transform,
    pub global_transform: GlobalTransform,
}

impl Transform2dBundle {
    /// Creates a new identity [`TransformBundle`], with no translation, rotation, and a scale of 1 on all axes.
    pub const fn identity() -> Self {
        Transform2dBundle {
            transform: Transform2d::identity(),
            z_offset: ZOffset(0.),
            transform_3d: Transform::identity(),
            global_transform: GlobalTransform::identity(),
        }
    }

    /// Creates a new [`TransformBundle`] from a [`Transform`].
    pub const fn from_transform(transform: Transform2d) -> Self {
        Transform2dBundle {
            transform,
            ..Transform2dBundle::identity()
        }
    }

    /// Returns this [`Transform2dBundle`] with a new Z offset.
    pub const fn with_z_offset(mut self, z_offset: ZOffset) -> Self {
        self.z_offset = z_offset;
        self
    }
}
