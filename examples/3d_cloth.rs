use bevy::prelude::*;
use bevy_verlet::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "3D cloth".to_string(),
                width: 1000.,
                height: 800.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(VerletPlugin::default())
        .add_startup_system(setup)
        .insert_resource(VerletConfig {
            sticks_computation_depth: 5,
            ..Default::default()
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-50., 0., -50.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let mesh = meshes.add(Mesh::from(shape::Cube::new(1.)));
    let stick_length: f32 = 2.;
    let (origin_x, origin_y) = (-5., 10.);
    let (points_x_count, points_y_count) = (20, 15);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(
                        origin_x + (2.0 * i as f32),
                        origin_y + (2.0 * (j + i / 2) as f32),
                        0.,
                    ),
                    ..Default::default()
                },
                VerletPoint::default(),
                Name::new(format!("Point {}", i)),
            ));
            if j == 0 {
                cmd.insert((VerletLocked, fixed_material.clone()));
            }
            entities.push(cmd.id());
        }
    }
    for (i, entity) in entities.iter().enumerate() {
        let above = i.checked_sub(points_x_count);
        let left = if i % points_x_count == 0 {
            None
        } else {
            i.checked_sub(1)
        };
        spawn_stick(&mut commands, *entity, &entities, stick_length, above);
        spawn_stick(&mut commands, *entity, &entities, stick_length, left);
    }
}

fn spawn_stick(
    commands: &mut Commands,
    entity: Entity,
    entities: &[Entity],
    length: f32,
    coord: Option<usize>,
) {
    if let Some(i) = coord {
        let other_entity = entities.get(i).unwrap();
        commands.spawn(VerletStick {
            point_a_entity: entity,
            point_b_entity: *other_entity,
            length,
        });
    }
}
