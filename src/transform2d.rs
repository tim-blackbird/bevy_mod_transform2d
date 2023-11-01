use bevy::{math::Affine2, prelude::*};

/// Describes the position of an [`Entity`] in 2d space.
///
/// This component acts as a proxy to the [`Transform`] component,
/// and thus *requires* that both a [`Transform`] and [`GlobalTransform`] are present to function.
///
/// If this [`Transform2d`] has a [`Parent`], then it's relative to the [`Transform2d`] or [`Transform`] of the [`Parent`].
#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component, PartialEq, Default)]
pub struct Transform2d {
    /// The translation along the `X` and `Y` axes.
    pub translation: Vec2,
    /// The rotation in radians. Positive values rotate anti-clockwise.
    pub rotation: f32,
    /// The scale along the `X` and `Y` axes.
    pub scale: Vec2,
    /// The translation along the `Z` axis.
    ///
    /// You might be surprised that 2D entities have a translation along the Z axis,
    /// but this third dimension is used when rendering to decide what should appear in front or behind.
    /// A higher translation on the Z axis puts the entity closer to the camera, and thus in front of entities with a lower Z translation.
    ///
    /// Keep in mind that this is relative to the [`Parent`]'s `z_translation`.
    /// The other fields on [`Transform2d`] don't affect this because they are strictly 2D.
    pub z_translation: f32,
}

impl Default for Transform2d {
    fn default() -> Self {
        Transform2d::IDENTITY
    }
}

impl Transform2d {
    /// Creates a new [`Transform2d`] at the position `(x, y)`.
    ///
    /// Rotation will be `0.` and scale will be `Vec2::ONE`
    #[inline]
    pub fn from_xy(x: f32, y: f32) -> Self {
        Transform2d::from_translation(Vec2::new(x, y))
    }

    /// Creates a new identity [`Transform2d`], with no translation, rotation, and a scale of 1 on all axes.
    ///
    /// Translation is `Vec2::ZERO`, rotation is `0.`, and scale is `Vec2::ONE`
    pub const IDENTITY: Self = Transform2d {
        translation: Vec2::ZERO,
        rotation: 0.,
        scale: Vec2::ONE,
        z_translation: 0.,
    };

    /// Creates a new [`Transform2d`] with `translation`.
    ///
    /// Rotation will be `0.` and scale will be `Vec2::ONE`
    #[inline]
    pub fn from_translation(translation: Vec2) -> Self {
        Transform2d {
            translation,
            ..Self::IDENTITY
        }
    }

    /// Creates a new [`Transform2d`] with `rotation`.
    ///
    /// Translation will be `Vec2::ZERO` and scale will be `Vec2::ONE`.
    #[inline]
    pub fn from_rotation(rotation: f32) -> Self {
        Transform2d {
            rotation,
            ..Self::IDENTITY
        }
    }

    /// Creates a new [`Transform2d`] with `scale`.
    ///
    /// Translation will be `Vec2::ZERO` and rotation will be `0.` .
    #[inline]
    pub fn from_scale(scale: impl IntoScale) -> Self {
        Transform2d {
            scale: scale.into_scale(),
            ..Self::IDENTITY
        }
    }

    /// Returns this [`Transform2d`] with a new translation.
    #[must_use]
    #[inline]
    pub fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    /// Returns this [`Transform2d`] with a new rotation.
    #[must_use]
    #[inline]
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Returns this [`Transform2d`] with a new scale.
    #[must_use]
    #[inline]
    pub fn with_scale(mut self, scale: impl IntoScale) -> Self {
        self.scale = scale.into_scale();
        self
    }

    /// Returns this [`Transform2d`] with a new Z translation.
    #[must_use]
    #[inline]
    pub fn with_z_translation(mut self, z_translation: f32) -> Self {
        self.z_translation = z_translation;
        self
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    pub fn local_x(&self) -> Vec2 {
        let (sin, cos) = self.rotation.sin_cos();
        (cos, sin).into()
    }

    #[inline]
    /// Equivalent to [`-local_x()`][Self::local_x()]
    pub fn left(&self) -> Vec2 {
        -self.local_x()
    }

    #[inline]
    /// Equivalent to [`local_x()`][Self::local_x()]
    pub fn right(&self) -> Vec2 {
        self.local_x()
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    pub fn local_y(&self) -> Vec2 {
        let (sin, cos) = self.rotation.sin_cos();
        (-sin, cos).into()
    }

    /// Equivalent to [`local_y()`][Self::local_y]
    #[inline]
    pub fn up(&self) -> Vec2 {
        self.local_y()
    }

    /// Equivalent to [`-local_y()`][Self::local_y]
    #[inline]
    pub fn down(&self) -> Vec2 {
        -self.local_y()
    }

    /// Returns the rotation matrix from this transforms rotation.
    #[inline]
    pub fn rotation_matrix(&self) -> Mat2 {
        Mat2::from_angle(self.rotation)
    }

    /// Computes the affine transformation matrix of this transform.
    #[inline]
    pub fn matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }

    /// Computes the affine transform of this transform.
    #[inline]
    pub fn affine(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }

    /// Translates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    #[inline]
    pub fn translate_around(&mut self, point: Vec2, angle: f32) {
        self.translation = point + Mat2::from_angle(angle) * (self.translation - point);
    }

    /// Rotates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    #[inline]
    pub fn rotate_around(&mut self, point: Vec2, angle: f32) {
        self.translate_around(point, angle);
        self.rotation += angle;
    }
}

impl From<Transform2d> for Transform {
    #[inline]
    fn from(transform2d: Transform2d) -> Self {
        Transform {
            translation: transform2d.translation.extend(transform2d.z_translation),
            rotation: Quat::from_rotation_z(transform2d.rotation),
            scale: transform2d.scale.extend(1.),
        }
    }
}

impl From<Transform> for Transform2d {
    fn from(transform_3d: Transform) -> Self {
        Transform2d {
            translation: transform_3d.translation.truncate(),
            rotation: transform_3d.rotation.to_euler(EulerRot::ZYX).0,
            scale: transform_3d.scale.truncate(),
            z_translation: transform_3d.translation.z,
        }
    }
}

pub trait IntoScale {
    fn into_scale(self) -> Vec2;
}

impl IntoScale for Vec2 {
    fn into_scale(self) -> Vec2 {
        self
    }
}

impl IntoScale for f32 {
    fn into_scale(self) -> Vec2 {
        Vec2::splat(self)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::TAU;

    use super::*;

    #[test]
    fn local_vectors() {
        let mut transform = Transform2d::from_rotation(TAU / 2.44);
        assert_eq!(transform.local_y(), transform.rotation_matrix() * Vec2::Y);
        assert_eq!(transform.local_x(), transform.rotation_matrix() * Vec2::X);
        transform.rotation = TAU / -0.56;
        assert_eq!(transform.local_y(), transform.rotation_matrix() * Vec2::Y);
        assert_eq!(transform.local_x(), transform.rotation_matrix() * Vec2::X);
    }
}
