#![allow(clippy::type_complexity)]

use bevy::{prelude::*, transform::TransformSystem};

#[cfg(feature = "bevy_rapier2d")]
use bevy_rapier2d::plugin::{systems::writeback_rigid_bodies, PhysicsStages, RapierPhysicsPlugin};

pub mod bundle;
pub mod systems;
pub mod transform_2d;
pub mod z_offset;

#[cfg(feature = "bevy_rapier2d")]
use systems::sync_from_3d_transform;
use systems::sync_to_3d_transform;
use transform_2d::Transform2d;
use z_offset::ZOffset;

pub mod prelude {
    pub use crate::{
        bundle::{Spatial2dBundle, Transform2dBundle},
        transform_2d::Transform2d,
        z_offset::ZOffset,
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
            .register_type::<ZOffset>()
            // Add transform2d sync system to startup so the first update is "correct"
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                sync_to_3d_transform.before(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                sync_to_3d_transform.before(TransformSystem::TransformPropagate),
            );

        #[cfg(feature = "bevy_rapier2d")]
        {
            if app.is_plugin_added::<RapierPhysicsPlugin>() {
                warn!("'bevy_rapier2d' feature enabled, but no compatible version of RapierPhysicsPlugin was not found. Make sure to add the RapierPhysicsPlugin before the Transform2dPlugin.");
            }
            app.add_system_to_stage(
                PhysicsStages::SyncBackend,
                sync_to_3d_transform.before(bevy::transform::transform_propagate_system),
            )
            .add_system_to_stage(
                PhysicsStages::Writeback,
                sync_from_3d_transform.after(writeback_rigid_bodies),
            );
        }
    }
}
