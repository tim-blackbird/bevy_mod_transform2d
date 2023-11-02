#![allow(clippy::type_complexity)]

use bevy::{prelude::*, transform::TransformSystem};

pub mod bundle;
pub mod systems;
pub mod transform2d;

use transform2d::Transform2d;

pub mod prelude {
    #[cfg(feature = "bevy_render")]
    pub use crate::bundle::Spatial2dBundle;
    pub use crate::{bundle::Transform2dBundle, transform2d::Transform2d, Transform2dPlugin};
}

/// The [`Plugin`] for [`Transform2d`].
///
/// This registers the systems that synchronise [`Transform2d`] with [`Transform`].
#[derive(Default)]
pub struct Transform2dPlugin;

/// A superset of the [`TransformSystem::TransformPropagate`] [`SystemSet`] that includes the systems that synchronise the [`Transform2d`] component.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Transform2dPropagate;

impl Plugin for Transform2dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2d>()
            // Add transform2d sync system to startup so the first update is "correct"
            .add_systems(
                PostStartup,
                systems::sync_transform_2d_to_3d
                    .before(TransformSystem::TransformPropagate)
                    .in_set(Transform2dPropagate),
            )
            .add_systems(
                PostUpdate,
                systems::sync_transform_2d_to_3d
                    .before(TransformSystem::TransformPropagate)
                    .in_set(Transform2dPropagate),
            );

        #[cfg(feature = "bevy_rapier2d")]
        {
            use bevy_rapier2d::plugin::{
                systems::writeback_rigid_bodies, PhysicsSet, RapierTransformPropagateSet,
            };

            app.add_systems(
                PostUpdate,
                (
                    systems::sync_transform_2d_to_3d
                        .in_set(PhysicsSet::SyncBackend)
                        .before(RapierTransformPropagateSet),
                    systems::sync_transform_3d_to_2d
                        .in_set(PhysicsSet::Writeback)
                        .after(writeback_rigid_bodies),
                ),
            );
        }

        #[cfg(feature = "bevy_xpbd_2d")]
        {
            use bevy_xpbd_2d::PhysicsSet;

            app.add_systems(
                PostUpdate,
                (
                    systems::sync_transform_2d_to_3d.in_set(PhysicsSet::Prepare),
                    systems::sync_transform_3d_to_2d.after(PhysicsSet::Sync),
                ),
            );
        }
    }
}
