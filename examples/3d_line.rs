use bevy::prelude::*;
use bevy_verlet::{BevyVerletPlugin, VerletConfig, VerletLocked, VerletPoint, VerletStick};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "3D line".to_string(),
            width: 1000.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyVerletPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_free_line)
        .add_startup_system(setup_fixed_line)
        .insert_resource(VerletConfig {
            sticks_computation_depth: 5,
            ..Default::default()
        })
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-30., 4., -80.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_free_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let mesh = meshes.add(Mesh::from(shape::Cube::new(1.)));
    let stick_length: f32 = 2.;
    let points_count = 10;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn_bundle(pbr_bundle(
            material.clone(),
            mesh.clone(),
            Vec3::new((i * 2) as f32, 20., 0.),
        ));
        cmd.insert(VerletPoint::default())
            .insert(Name::new(format!("Point {}", i)));
        if previous_entity.is_none() {
            cmd.insert(VerletLocked {}).insert(fixed_material.clone());
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands
                .spawn()
                .insert(VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                })
                .insert(Name::new(format!("Stick {}", i)));
        }
        previous_entity = Some(entity);
    }
}

fn setup_fixed_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let mesh = meshes.add(Mesh::from(shape::Cube::new(1.)));
    let stick_length: f32 = 2.;
    let points_count = 20;
    let start_pos = -10.;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn_bundle(pbr_bundle(
            material.clone(),
            mesh.clone(),
            Vec3::new(start_pos + (i * 2) as f32, 0., 0.),
        ));
        cmd.insert(VerletPoint::default())
            .insert(Name::new(format!("Point {}", i)));
        if previous_entity.is_none() || i == points_count {
            cmd.insert(VerletLocked {}).insert(fixed_material.clone());
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands
                .spawn()
                .insert(VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                })
                .insert(Name::new(format!("Stick {}", i)));
        }
        previous_entity = Some(entity);
    }
}

fn pbr_bundle(material: Handle<StandardMaterial>, mesh: Handle<Mesh>, pos: Vec3) -> PbrBundle {
    PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(pos),
        ..Default::default()
    }
}
