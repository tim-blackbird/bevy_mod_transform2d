use bevy::{math::Vec3Swizzles, prelude::*};

use crate::prelude::{Transform2d, ZOffset};

pub fn sync_to_3d_transform(
    mut query: Query<
        (&Transform2d, Option<&ZOffset>, &mut Transform),
        Or<(Changed<Transform2d>, Changed<ZOffset>)>,
    >,
) {
    for (transform_2d, z_offset, mut transform_3d) in &mut query {
        // Translation
        transform_3d.translation = if let Some(z_offset) = z_offset {
            transform_2d.translation.extend(z_offset.0)
        } else {
            // Keep the transforms current Z position when the ZOffset component is not present
            // to avoid needless surprises.
            transform_2d.translation.extend(transform_3d.translation.z)
        };

        // Rotation. Rotations around the axes other than Z are not kept.
        transform_3d.rotation = Quat::from_rotation_z(transform_2d.rotation);

        // Scale. The Z axis is always set to 1 as any other value will affect the Z position of children
        // which will in turn affect their render order.
        transform_3d.scale = transform_2d.scale.extend(1.);
    }
}

pub fn sync_from_3d_transform(
    mut query: Query<(&mut Transform2d, &Transform), Changed<Transform>>,
) {
    for (mut transform_2d, transform_3d) in &mut query {
        // Translation
        transform_2d.translation = transform_3d.translation.xy();

        // Rotation
        let (z, _y, _x) = transform_3d.rotation.to_euler(EulerRot::ZYX);
        transform_2d.rotation = z;

        // Scale
        transform_2d.scale = transform_3d.scale.truncate();
    }
}
