use bevy::prelude::*;

use crate::{transform2d::Transform2d};

#[derive(Bundle, Clone, Copy, Debug, Default, Reflect, FromReflect)]
pub struct Transform2dBundle {
    pub transform: Transform2d,
    pub transform_3d: Transform,
    pub global_transform: GlobalTransform,
}

impl Transform2dBundle {
    /// Creates a new identity [`TransformBundle`], with no translation, rotation, and a scale of 1 on all axes.
    pub const IDENTITY: Self = Transform2dBundle {
        transform: Transform2d::IDENTITY,
        transform_3d: Transform::IDENTITY,
        global_transform: GlobalTransform::IDENTITY,
    };

    /// Creates a new [`TransformBundle`] from a [`Transform`].
    pub const fn from_transform(transform: Transform2d) -> Self {
        Transform2dBundle {
            transform,
            ..Transform2dBundle::IDENTITY
        }
    }
}

/// A [`Bundle`] with the following [`Component`](bevy_ecs::component::Component)s:
/// * [`Visibility`] and [`ComputedVisibility`], which describe the visibility of an entity
/// * [`Transform`] and [`GlobalTransform`], which describe the position of an entity
///
/// * To show or hide an entity, you should set its [`Visibility`].
/// * To get the computed visibility of an entity, you should get its [`ComputedVisibility`].
/// * To place or move an entity, you should set its [`Transform`].
/// * To get the global transform of an entity, you should get its [`GlobalTransform`].
/// * For hierarchies to work correctly, you must have all four components.
///   * You may use the [`SpatialBundle`] to guarantee this.
#[derive(Bundle, Debug, Default)]
pub struct Spatial2dBundle {
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The computed visibility of the entity.
    pub computed: ComputedVisibility,
    /// The transform of the entity.
    pub transform: Transform2d,
    /// The 3D transform of the entity.
    pub transform_3d: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}

impl Spatial2dBundle {
    /// Creates a new [`Spatial2dBundle`] from a [`Transform2d`].
    ///
    /// This initializes [`GlobalTransform`] as identity, and visibility as visible.
    #[inline]
    pub const fn from_transform(transform: Transform2d) -> Self {
        Spatial2dBundle {
            transform,
            ..Self::VISIBLE_IDENTITY
        }
    }

    /// A visible [`Spatial2dBundle`], with no translation, rotation, and a scale of 1 on all axes.
    pub const VISIBLE_IDENTITY: Self = Spatial2dBundle {
        visibility: Visibility::VISIBLE,
        computed: ComputedVisibility::INVISIBLE,
        transform: Transform2d::IDENTITY,
        transform_3d: Transform::IDENTITY,
        global_transform: GlobalTransform::IDENTITY,
    };

    /// An invisible [`Spatial2dBundle`], with no translation, rotation, and a scale of 1 on all axes.
    pub const INVISIBLE_IDENTITY: Self = Spatial2dBundle {
        visibility: Visibility::INVISIBLE,
        ..Self::VISIBLE_IDENTITY
    };
}

impl From<Transform2d> for Spatial2dBundle {
    #[inline]
    fn from(transform: Transform2d) -> Self {
        Self::from_transform(transform)
    }
}
