use crate::components::Position;
use derive_deref::{Deref, DerefMut};
use legion::{world::SubWorld, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct RemoveBuffer(VecDeque<RigidBodyHandle>);

impl RemoveBuffer {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, handle: RigidBodyHandle) {
        self.0.push_back(handle);
    }
}

#[derive(Debug, Clone)]
pub struct FixedBody;

#[derive(Debug, Clone)]
pub struct Gravity(Vector<f32>);

pub fn setup_resources(resources: &mut Resources) {
    resources.insert(Gravity(vector![0.0, 0.0, -9.81]));
    resources.insert(IntegrationParameters::default());
    resources.insert(RigidBodySet::new());
    resources.insert(ColliderSet::new());
    resources.insert(PhysicsPipeline::new());
    resources.insert(IslandManager::new());
    resources.insert(BroadPhase::new());
    resources.insert(NarrowPhase::new());
    resources.insert(ImpulseJointSet::new());
    resources.insert(MultibodyJointSet::new());
    resources.insert(CCDSolver::new());
    resources.insert(RemoveBuffer::new());
}

#[system]
#[read_component(RigidBodyHandle)]
#[write_component(Position)]
pub fn update_physics(
    world: &mut SubWorld,
    #[resource] gravity: &Gravity,
    #[resource] integration_parameters: &IntegrationParameters,
    #[resource] rigid_body_set: &mut RigidBodySet,
    #[resource] collider_set: &mut ColliderSet,
    #[resource] physics_pipeline: &mut PhysicsPipeline,
    #[resource] island_manager: &mut IslandManager,
    #[resource] broad_phase: &mut BroadPhase,
    #[resource] narrow_phase: &mut NarrowPhase,
    #[resource] impulse_joint_set: &mut ImpulseJointSet,
    #[resource] multibody_joint_set: &mut MultibodyJointSet,
    #[resource] ccd_solver: &mut CCDSolver,
    #[resource] remove_buffer: &mut RemoveBuffer,
) {
    physics_pipeline.step(
        &gravity.0,
        integration_parameters,
        island_manager,
        broad_phase,
        narrow_phase,
        rigid_body_set,
        collider_set,
        impulse_joint_set,
        multibody_joint_set,
        ccd_solver,
        &(),
        &(),
    );

    update_positions(world, rigid_body_set, island_manager);
    cleanup_old(
        remove_buffer,
        rigid_body_set,
        island_manager,
        collider_set,
        impulse_joint_set,
        multibody_joint_set,
    );
}

fn update_positions(
    world: &mut SubWorld,
    rigid_body_set: &mut RigidBodySet,
    island_manager: &IslandManager,
) {
    let mut bodies = <(&mut Position, &RigidBodyHandle)>::query().filter(!component::<FixedBody>());
    let mut handle_pos_map: std::collections::HashMap<_, _> = bodies
        .iter_mut(world)
        .map(|(pos, handle)| (handle, pos))
        .collect();

    for handle in island_manager.active_dynamic_bodies() {
        if let Some(pos) = handle_pos_map.get_mut(handle) {
            if let Some(body) = rigid_body_set.get(*handle) {
                ***pos = Vec3::from(*body.translation());
            }
        }
    }
}

fn cleanup_old(
    remove_buffer: &mut RemoveBuffer,
    rigid_body_set: &mut RigidBodySet,
    island_manager: &mut IslandManager,
    collider_set: &mut ColliderSet,
    impulse_joint_set: &mut ImpulseJointSet,
    multibody_joint_set: &mut MultibodyJointSet,
) {
    const MAX_REMOVE: usize = 100;

    if remove_buffer.len() == 0 {
        return;
    }

    let drain_len = MAX_REMOVE.min(remove_buffer.len());

    for handle in remove_buffer.drain(0..drain_len) {
        rigid_body_set.remove(
            handle,
            island_manager,
            collider_set,
            impulse_joint_set,
            multibody_joint_set,
            true,
        );
    }
}
