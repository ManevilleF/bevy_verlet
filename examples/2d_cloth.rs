use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_verlet::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D cloth".to_string(),
                resolution: (1000., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VerletPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    let stick_length: f32 = 35.;
    let (origin_x, origin_y) = (-450., 350.);
    let (points_x_count, points_y_count) = (30, 15);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn((
                Sprite {
                    color: if j == 0 { RED.into() } else { WHITE.into() },
                    custom_size: Some(Vec2::splat(10.)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    origin_x + (30. * i as f32),
                    origin_y + (-30. * (j + i / 3) as f32),
                    0.,
                ),
                VerletPoint::new(0.2),
                Name::new(format!("Point {}", i)),
            ));
            if j == 0 {
                cmd.insert(VerletLocked);
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
