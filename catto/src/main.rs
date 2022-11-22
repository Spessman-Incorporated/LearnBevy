extern crate core;

use std::f32::consts::*;
use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::BLUE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let catto_handle = asset_server.load("models/catto.gltf#Mesh0/Primitive0");

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 0.9).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/catto.gltf#Scene0"),
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

