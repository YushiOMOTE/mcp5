use crate::terrain::Loader;
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use nalgebra::Vector3;

#[derive(Component)]
pub struct Player;

pub fn create_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    commands
        .spawn()
        .insert(Player)
        .insert(Loader::new())
        .insert(RigidBody::Dynamic)
        .insert(ExternalImpulse::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(GravityScale(3.0))
        .insert(Ccd::enabled())
        .insert(Velocity::zero())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
            material: materials.add(Color::RED.into()),
            transform,
            ..default()
        });
}

pub fn create_camera(commands: &mut Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub fn update_camera_system(
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<(&Camera, &mut Transform), Without<Player>>,
) {
    for (_, player_pos) in &players {
        for (_, mut camera_pos) in &mut cameras {
            *camera_pos =
                Transform::from_translation(player_pos.translation + Vec3::new(4.0, 20.0, 8.0))
                    .looking_at(player_pos.translation, Vec3::Y);
        }
    }
}

pub fn input_control_system(
    mut query: Query<(
        &mut Velocity,
        &mut ExternalImpulse,
        &Player,
        &RapierRigidBodyHandle,
    )>,
    context: Res<RapierContext>,
    input: Res<Input<KeyCode>>,
) {
    for (mut velocity, mut impulse, _, handle) in &mut query {
        let step = 10.0;

        if input.pressed(KeyCode::W) {
            velocity.linvel.z = -1.0 * step;
        }
        if input.just_released(KeyCode::W) {
            velocity.linvel.z = 0.0;
        }

        if input.pressed(KeyCode::A) {
            velocity.linvel.x = -1.0 * step;
        }
        if input.just_released(KeyCode::A) {
            velocity.linvel.x = 0.0;
        }

        if input.pressed(KeyCode::S) {
            velocity.linvel.z = step;
        }
        if input.just_released(KeyCode::S) {
            velocity.linvel.z = 0.0;
        }

        if input.pressed(KeyCode::D) {
            velocity.linvel.x = step;
        }
        if input.just_released(KeyCode::D) {
            velocity.linvel.x = 0.0;
        }

        if input.pressed(KeyCode::J) {
            let body = match context.bodies.get(handle.0) {
                Some(b) => b,
                None => continue,
            };
            if vertically_stable(&body) {
                impulse.impulse = Vec3::new(0.0, 10.0, 0.0);
            }
        }
    }
}

fn vertically_stable(body: &rapier3d::prelude::RigidBody) -> bool {
    let e1 = body.gravitational_potential_energy(0.001, Vector3::new(0.0, -9.81, 0.0));
    let e2 = body.gravitational_potential_energy(0.002, Vector3::new(0.0, -9.81, 0.0));
    e1 == e2
}
