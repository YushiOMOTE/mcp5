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
        .insert(Collider::cuboid(0.4, 0.4, 0.4))
        .insert(GravityScale(3.0))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.8, 0.8, 0.8).into()),
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
        &mut Transform,
        &mut ExternalImpulse,
        &Player,
        &RapierRigidBodyHandle,
    )>,
    context: Res<RapierContext>,
    input: Res<Input<KeyCode>>,
) {
    for (mut transform, mut impulse, _, handle) in &mut query {
        if input.pressed(KeyCode::W) {
            transform.translation.z -= 0.2;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 0.2;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.z += 0.2;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 0.2;
        }

        if input.pressed(KeyCode::J) {
            let body = match context.bodies.get(handle.0) {
                Some(b) => b,
                None => continue,
            };
            if horizontally_stable(&body) {
                impulse.impulse = Vec3::new(0.0, 5.0, 0.0);
            }
        }
    }
}

fn horizontally_stable(body: &rapier3d::prelude::RigidBody) -> bool {
    let e1 = body.gravitational_potential_energy(0.001, Vector3::new(0.0, -9.81, 0.0));
    let e2 = body.gravitational_potential_energy(0.002, Vector3::new(0.0, -9.81, 0.0));
    e1 == e2
}
