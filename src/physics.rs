use crate::components::Position;
use legion::{world::SubWorld, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;

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

// pub fn position(&self, handle: PhysicsHandle) -> Position {
//     let vec = self
//         .get()
//         .rigid_body_set
//         .get(handle.rigid_body_handle)
//         .unwrap()
//         .translation();

//     Position(Vec3::from(*vec))
// }

// struct Storage {
//     rigid_body_set: RigidBodySet,
//     collider_set: ColliderSet,
// }

// struct Engine {
//     physics_pipeline: PhysicsPipeline,
//     island_manager: IslandManager,
//     broad_phase: BroadPhase,
//     narrow_phase: NarrowPhase,
//     impulse_joint_set: ImpulseJointSet,
//     multibody_joint_set: MultibodyJointSet,
//     ccd_solver: CCDSolver,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct PhysicsHandle {
//     rigid_body_handle: RigidBodyHandle,
//     collider_handle: ColliderHandle,
// }

// pub struct PhysicsWorld {
//     gravity: Vector<f32>,
//     integration_parameters: IntegrationParameters,
//     set: Option<Storage>,
//     inner: Option<Engine>,
// }

// impl PhysicsWorld {
//     pub fn new() -> Self {
//         Self {
//             gravity: vector![0.0, 0.0, 9.81 * 10.0],
//             integration_parameters: IntegrationParameters::default(),
//             set: Some(Storage {
//                 rigid_body_set: RigidBodySet::new(),
//                 collider_set: ColliderSet::new(),
//             }),
//             inner: Some(Engine {
//                 physics_pipeline: PhysicsPipeline::new(),
//                 island_manager: IslandManager::new(),
//                 broad_phase: BroadPhase::new(),
//                 narrow_phase: NarrowPhase::new(),
//                 impulse_joint_set: ImpulseJointSet::new(),
//                 multibody_joint_set: MultibodyJointSet::new(),
//                 ccd_solver: CCDSolver::new(),
//             }),
//         }
//     }

//     pub fn create_dynamic_body(&mut self, pos: Position, size: Size) -> PhysicsHandle {
//         self.create(pos, size, RigidBodyType::Dynamic)
//     }

//     pub fn create_fixed_body(&mut self, pos: Position, size: Size) -> PhysicsHandle {
//         self.create(pos, size, RigidBodyType::Fixed)
//     }

//     pub fn position(&self, handle: PhysicsHandle) -> Position {
//         let vec = self
//             .get()
//             .rigid_body_set
//             .get(handle.rigid_body_handle)
//             .unwrap()
//             .translation();

//         Position(Vec3::from(*vec))
//     }

//     pub fn set_pos(&mut self, handle: PhysicsHandle, pos: Position) {
//         self.update(|rigid_body_set, _| {
//             let body = match rigid_body_set.get_mut(handle.rigid_body_handle) {
//                 Some(body) => body,
//                 None => return,
//             };

//             body.set_next_kinematic_translation((*pos).into());
//         });
//     }

//     pub fn add_force(&mut self, handle: PhysicsHandle, dir: Vec3) {
//         self.update(|rigid_body_set, _| {
//             let body = match rigid_body_set.get_mut(handle.rigid_body_handle) {
//                 Some(body) => body,
//                 None => return,
//             };

//             // println!("Entity(16): add torque: {:?}: {:?}", dir, handle);
//             // body.reset_forces(true);
//             body.apply_impulse(dir.into(), true);
//         });
//     }

//     fn create(&mut self, pos: Position, size: Size, body_type: RigidBodyType) -> PhysicsHandle {
//         let (rigid_body_handle, collider_handle) = self.update(|rigid_body_set, collider_set| {
//             let collider = ColliderBuilder::cuboid(size.x * 0.8, size.y * 0.8, size.z * 0.8)
//                 .mass(100.0)
//                 .build();

//             let rigid_body = RigidBodyBuilder::new(body_type)
//                 .translation(vector![pos.x, pos.y, pos.z])
//                 .enabled_rotations(false, false, true)
//                 .build();
//             let rigid_body_handle = rigid_body_set.insert(rigid_body);

//             let collider_handle =
//                 collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

//             (rigid_body_handle, collider_handle)
//         });

//         PhysicsHandle {
//             rigid_body_handle,
//             collider_handle,
//         }
//     }

//     pub fn step(&mut self) {
//         let Storage {
//             mut rigid_body_set,
//             mut collider_set,
//         } = self.set.take().unwrap();

//         let Engine {
//             mut physics_pipeline,
//             mut island_manager,
//             mut broad_phase,
//             mut narrow_phase,
//             mut impulse_joint_set,
//             mut multibody_joint_set,
//             mut ccd_solver,
//         } = self.inner.take().unwrap();

//         physics_pipeline.step(
//             &self.gravity,
//             &self.integration_parameters,
//             &mut island_manager,
//             &mut broad_phase,
//             &mut narrow_phase,
//             &mut rigid_body_set,
//             &mut collider_set,
//             &mut impulse_joint_set,
//             &mut multibody_joint_set,
//             &mut ccd_solver,
//             &(),
//             &(),
//         );

//         self.set = Some(Storage {
//             rigid_body_set,
//             collider_set,
//         });

//         self.inner = Some(Engine {
//             physics_pipeline,
//             island_manager,
//             broad_phase,
//             narrow_phase,
//             impulse_joint_set,
//             multibody_joint_set,
//             ccd_solver,
//         });
//     }

//     fn get(&self) -> &Storage {
//         self.set.as_ref().unwrap()
//     }

//     fn update<T, F: FnMut(&mut RigidBodySet, &mut ColliderSet) -> T>(&mut self, mut f: F) -> T {
//         let Storage {
//             mut rigid_body_set,
//             mut collider_set,
//         } = self.set.take().unwrap();

//         let ret = f(&mut rigid_body_set, &mut collider_set);

//         self.set = Some(Storage {
//             rigid_body_set,
//             collider_set,
//         });

//         ret
//     }
// }
