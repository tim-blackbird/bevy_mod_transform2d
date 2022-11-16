#![allow(clippy::type_complexity)]

use bevy::{prelude::*, transform::TransformSystem};

pub mod bundle;
pub mod systems;
pub mod transform2d;

use transform2d::Transform2d;

use crate::systems::sync_2d_to_3d_transform;

pub mod prelude {
    pub use crate::{
        bundle::{Spatial2dBundle, Transform2dBundle},
        transform2d::Transform2d,
        Transform2dPlugin,
    };
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
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                sync_2d_to_3d_transform.before(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                sync_2d_to_3d_transform.before(TransformSystem::TransformPropagate),
            );

        #[cfg(feature = "bevy_rapier2d")]
        {
            use bevy_rapier2d::plugin::{
                systems::writeback_rigid_bodies, PhysicsStages, RapierPhysicsPlugin,
            };
            use systems::sync_3d_to_2d_transform;

            if app.is_plugin_added::<RapierPhysicsPlugin>() {
                warn!("'bevy_rapier2d' feature enabled, but no compatible version of RapierPhysicsPlugin was not found. Make sure to add the RapierPhysicsPlugin before the Transform2dPlugin.");
            }

            app.add_system_to_stage(
                PhysicsStages::SyncBackend,
                sync_2d_to_3d_transform.before(bevy::transform::transform_propagate_system),
            )
            .add_system_to_stage(
                PhysicsStages::Writeback,
                sync_3d_to_2d_transform.after(writeback_rigid_bodies),
            );
        }
    }
}
