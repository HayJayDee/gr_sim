use bevy::prelude::*;
use coordinate_system::{spherical_to_cartesian, SphericalCoordinates};
use std::f32::consts::PI;

pub mod coordinate_system;

#[derive(Component)]
struct Planet {
    pub sphere_coords: SphericalCoordinates,
}

fn setup_camera(mut commands: Commands) {
    
    let mut transform = Transform::default();
    transform.rotate_axis(Vec3::X, -PI/2.0);
    transform.translation += Vec3::new(0.0,10.0,0.0);
    commands.spawn(Camera3dBundle {
        transform: transform,
        ..Default::default()
    });
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planet = Planet {
        sphere_coords: SphericalCoordinates {
            radius: 1.0,
            theta: PI / 2.0,
            phi: 0.0,
        },
    };

    let planet_coords = Vec3::from_array(coordinate_system::spherical_to_cartesian(
        &planet.sphere_coords,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(planet_coords),
            ..Default::default()
        },
        planet,
    ));
    commands.spawn(DirectionalLightBundle {
        transform: Transform::default(),
        ..Default::default()
    });
}

fn update_planet(time: Res<Time>, mut planets: Query<(&mut Planet, &mut Transform)>) {
    for (mut planet, mut transform) in &mut planets {
        planet.sphere_coords.phi += time.delta_seconds();
        transform.translation = Vec3::from_array(spherical_to_cartesian(
            &planet.sphere_coords,
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, update_planet)
        .run();
}
