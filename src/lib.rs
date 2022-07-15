#![allow(clippy::type_complexity)]

use bevy::{prelude::*, reflect::Reflect, transform::TransformSystem};

#[cfg(feature = "bevy_rapier2d")]
use bevy_rapier2d::{
    pipeline::CollisionEvent,
    plugin::{systems::writeback_rigid_bodies, PhysicsStages},
};

pub mod systems;
mod transform_2d;
mod z_index;

#[cfg(feature = "bevy_rapier2d")]
use systems::sync_from_3d_transform;
use systems::sync_to_3d_transform;
use transform_2d::Transform2d;
use z_index::ZIndex;

pub mod prelude {
    pub use crate::{
        transform_2d::Transform2d, z_index::ZIndex, Transform2dBundle, Transform2dPlugin,
    };
}

#[derive(Bundle, Clone, Copy, Debug, Default, Reflect)]
pub struct Transform2dBundle {
    pub transform_2d: Transform2d,
    pub z_index: ZIndex,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub struct Transform2dPlugin;

impl Plugin for Transform2dPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2d>()
            .register_type::<ZIndex>()
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
            // Check if the RapierPhysicsPlugin plugin was added by confirming the Events::<CollisionEvent> resource exists.
            // Checking for Events::<CollisionEvent> because it is unlikely a user of bevy_rapier would insert that themselfves.
            if app
                .world
                .get_resource::<bevy::ecs::event::Events<CollisionEvent>>()
                .is_none()
            {
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
