#![allow(clippy::type_complexity)]

use bevy::{prelude::*, transform::TransformSystem};

pub mod bundle;
pub mod systems;
pub mod transform2d;

use transform2d::Transform2d;

use crate::systems::sync_transform_2d_to_3d;

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

impl Plugin for Transform2dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2d>()
            // Add transform2d sync system to startup so the first update is "correct"
            .add_startup_system(
                sync_transform_2d_to_3d
                    .in_base_set(StartupSet::PostStartup)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_system(
                sync_transform_2d_to_3d
                    .in_base_set(CoreSet::PostUpdate)
                    .before(TransformSystem::TransformPropagate),
            );

        #[cfg(feature = "bevy_rapier2d")]
        {
            use bevy_rapier2d::{
                pipeline::CollisionEvent,
                plugin::{
                    systems::writeback_rigid_bodies, PhysicsSet, RapierTransformPropagateSet,
                },
            };
            use systems::sync_transform_3d_to_2d;

            if app
                .world
                .get_resource::<bevy::ecs::event::Events<CollisionEvent>>()
                .is_none()
            {
                panic!(
                    "The 'bevy_rapier2d' feature is enabled, but no compatible version of RapierPhysicsPlugin was not found. \
                    Make sure to add the Transform2dPlugin after the RapierPhysicsPlugin."
                );
            }

            app.add_system(
                sync_transform_2d_to_3d
                    .in_base_set(PhysicsSet::SyncBackend)
                    .before(RapierTransformPropagateSet),
            )
            .add_system(
                sync_transform_3d_to_2d
                    .in_base_set(PhysicsSet::Writeback)
                    .after(writeback_rigid_bodies),
            );
        }
    }
}
