use bevy::prelude::*;
use bevy_verlet::{VerletLocked, VerletPlugin, VerletPoint, VerletStick};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "2D cloth".to_string(),
            width: 1000.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(VerletPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let stick_length: f32 = 35.;
    let (origin_x, origin_y) = (-450., 350.);
    let (points_x_count, points_y_count) = (30, 15);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: if j == 0 { Color::RED } else { Color::WHITE },
                    custom_size: Some(Vec2::splat(10.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    origin_x + (30. * i as f32),
                    origin_y + (-30. * (j + i / 3) as f32),
                    0.,
                ),
                ..Default::default()
            });
            cmd.insert(VerletPoint::default())
                .insert(Name::new(format!("Point {}", i)));
            if j == 0 {
                cmd.insert(VerletLocked);
            }
            entities.push(cmd.id());
        }
    }
    for (i, entity) in entities.iter().enumerate() {
        let above = i.checked_sub(points_x_count);
        let left = (i % points_x_count == 0).then(|| i - 1);
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
        commands.spawn().insert(VerletStick {
            point_a_entity: entity,
            point_b_entity: *other_entity,
            length,
        });
    }
}
