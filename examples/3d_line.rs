use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_verlet::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D line".to_string(),
                resolution: (1000., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VerletPlugin::default())
        .add_systems(Startup, (setup_camera, setup_free_line, setup_fixed_line))
        .insert_resource(VerletConfig {
            sticks_computation_depth: 5,
            ..Default::default()
        })
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-30., 4., -80.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_free_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::from(WHITE));
    let fixed_material = materials.add(Color::from(RED));
    let mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let stick_length: f32 = 2.;
    let points_count = 10;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn((
            MeshMaterial3d(material.clone()),
            Mesh3d(mesh.clone()),
            Transform::from_xyz((i * 2) as f32, 20., 0.),
            VerletPoint::default(),
            Name::new(format!("Point {}", i)),
        ));
        if previous_entity.is_none() {
            cmd.insert((VerletLocked, fixed_material.clone()));
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands.spawn((
                VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                },
                Name::new(format!("Stick {}", i)),
            ));
        }
        previous_entity = Some(entity);
    }
}

fn setup_fixed_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::from(WHITE));
    let fixed_material = materials.add(Color::from(RED));
    let mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let stick_length: f32 = 2.;
    let points_count = 20;
    let start_pos = -10.;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn((
            MeshMaterial3d(material.clone()),
            Mesh3d(mesh.clone()),
            Transform::from_xyz(start + (i * 2) as f32, 0., 0.),
            VerletPoint::default(),
            Name::new(format!("Point {}", i)),
        ));
        if previous_entity.is_none() || i == points_count {
            cmd.insert((VerletLocked, fixed_material.clone()));
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands.spawn((
                VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                },
                Name::new(format!("Stick {}", i)),
            ));
        }
        previous_entity = Some(entity);
    }
}
