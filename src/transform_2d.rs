use bevy::{math::Mat2, prelude::*, reflect::FromReflect};

/// Describes the position of an [`Entity`] in 2d space.
///
/// This component acts as a proxy to the [`Transform`] component,
/// and thus *requires* that both a [`Transform`] and [`GlobalTransform`] are present to function.
///
/// If this [`Transform2d`] has a parent, then it's relative to the [`Transform2d`] or [`Transform`] of the parent.
#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component, PartialEq, Default)]
pub struct Transform2d {
    /// The translation along the `X` and `Y` axes.
    pub translation: Vec2,
    /// The rotation in radians. Positive values rotate anti-clockwise.
    pub rotation: f32,
    /// The scale along the `X` and `Y` axes.
    pub scale: Vec2,
}

impl Default for Transform2d {
    fn default() -> Self {
        Transform2d::identity()
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
    /// Translation will be `Vec2::ZERO`, rotation will be `0.` and scale will be `Vec2::ONE`
    #[inline]
    pub const fn identity() -> Self {
        Transform2d {
            translation: Vec2::ZERO,
            rotation: 0.,
            scale: Vec2::ONE,
        }
    }

    /// Creates a new [`Transform2d`] with `translation`.
    ///
    /// Rotation will be `0.` and scale will be `Vec2::ONE`
    #[inline]
    pub fn from_translation(translation: Vec2) -> Self {
        Transform2d {
            translation,
            ..Self::identity()
        }
    }

    /// Creates a new [`Transform2d`] with `rotation`.
    ///
    /// Translation will be `Vec2::ZERO` and scale will be `Vec2::ONE`.
    #[inline]
    pub fn from_rotation(rotation: f32) -> Self {
        Transform2d {
            rotation,
            ..Self::identity()
        }
    }

    /// Creates a new [`Transform2d`] with `scale`.
    ///
    /// Translation will be `Vec2::ZERO` and rotation will be `0.` .
    #[inline]
    pub fn from_scale(scale: Vec2) -> Self {
        Transform2d {
            scale,
            ..Self::identity()
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
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    pub fn local_x(&self) -> Vec2 {
        self.rotation_matrix() * Vec2::X
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
        self.rotation_matrix() * Vec2::Y
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
