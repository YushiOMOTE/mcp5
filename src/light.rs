use bevy::prelude::*;

pub fn create_light(commands: &mut Commands) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 50000.0,
            range: 500.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 80.0, 0.0),
        ..default()
    });
}
