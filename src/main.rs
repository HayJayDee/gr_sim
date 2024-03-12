use bevy::prelude::*;
use coordinate_system::{spherical_to_cartesian, SphericalCoordinates};
use ndarray::Array1;
use ode_solver::euler_solver::EulerSolver;
use ode_solver::runge_kutta::RungeKutta4;
use std::f32::consts::PI;

use crate::ode_solver::solver::Solver;

pub mod coordinate_system;
pub mod ode_solver;

#[derive(Component)]
struct Planet {
    pub sphere_coords: SphericalCoordinates,
}


#[derive(Component)]
struct SolverComponent {
    pub euler_solver: EulerSolver,
    pub runge_solver: RungeKutta4<1>
}

fn setup_camera(mut commands: Commands) {
    
    let mut transform = Transform::default();
    transform.translation += Vec3::new(0.0,0.0,10.0);
    commands.spawn(Camera3dBundle {
        transform,
        ..Default::default()
    });

    commands.spawn(SolverComponent {
        euler_solver: EulerSolver::new(0.0, 100.0),
        runge_solver: RungeKutta4::<1> {
            time: 0.0,
            state: Array1::from_iter([100.0])
        }
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

fn update_temp(time: Res<Time>, mut solvers: Query<&mut SolverComponent>) {
    for mut solver in &mut solvers {
        solver.euler_solver.next_step(&|_time, curr_t| {
            -0.07 * (curr_t - 20.0)
        }, time.delta_seconds());
        
        solver.runge_solver.next_step(&|_time, curr_t| {
            -0.07 * (curr_t - 20.0)
        }, time.delta_seconds());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, update_planet)
        .add_systems(Update, update_temp)
        .run();
}
