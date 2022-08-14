use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod chunk;
mod light;
mod map;
mod player;
mod terrain;
mod voxel;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    light::create_light(&mut commands);

    player::create_camera(&mut commands);

    player::create_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 50.0, 0.0),
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(player::input_control_system)
        .add_system(player::update_camera_system)
        .add_system(terrain::request_terrain_system)
        .add_system(terrain::render_terrain_system)
        .run();
}
