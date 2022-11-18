use bevy::prelude::*;

use crate::transform2d::Transform2d;

// FIXME: Pretty sure the Changed filter won't help when both these systems are running as one will trigger change detection for the other.

pub fn sync_transform_2d_to_3d(
    mut query: Query<(&Transform2d, &mut Transform), Changed<Transform2d>>,
) {
    for (&transform_2d, mut transform_3d) in &mut query {
        *transform_3d = transform_2d.into();
    }
}

pub fn sync_transform_3d_to_2d(
    mut query: Query<(&mut Transform2d, &Transform), Changed<Transform>>,
) {
    for (mut transform_2d, &transform_3d) in &mut query {
        *transform_2d = transform_3d.into();
    }
}
