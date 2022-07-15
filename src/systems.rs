use bevy::{math::Vec3Swizzles, prelude::*, tasks::ComputeTaskPool};

use crate::prelude::{Transform2d, ZIndex};

// TODO: What is a good batch size?
const BATCH_SIZE: usize = 32;

pub fn sync_to_3d_transform(
    mut query: Query<
        (&Transform2d, Option<&ZIndex>, &mut Transform),
        Or<(Changed<Transform2d>, Changed<ZIndex>)>,
    >,
    pool: Res<ComputeTaskPool>,
) {
    query.par_for_each_mut(
        &pool,
        BATCH_SIZE,
        |(transform_2d, z_index, mut transform)| {
            // Translation
            let [x, y] = transform_2d.translation.to_array();
            transform.translation = if let Some(z_index) = z_index {
                Vec3::new(x, y, z_index.0)
            } else {
                Vec3::new(x, y, transform.translation.z)
            };

            // Rotation
            transform.rotation = Quat::from_rotation_z(transform_2d.rotation);

            // Scale
            transform.scale = transform_2d.scale.extend(transform.scale.z);
        },
    );
}

// TODO: Untested.
pub fn sync_from_3d_transform(
    mut query: Query<(&mut Transform2d, &Transform), Changed<Transform>>,
    pool: Res<ComputeTaskPool>,
) {
    query.par_for_each_mut(&pool, BATCH_SIZE, |(mut transform_2d, transform)| {
        // Translation
        transform_2d.translation = transform.translation.xy();

        // Rotation
        let (_y, _x, z) = transform.rotation.to_euler(EulerRot::YXZ);
        transform_2d.rotation = z;

        // Scale
        transform_2d.scale = transform.scale.truncate();
    });
}
