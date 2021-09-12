use bevy::prelude::*;
use bevy_verlet::{
    BevyVerletPlugin, VerletLocked, VerletPoint2, VerletPointSpriteBundle, VerletStick,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyVerletPlugin::default())
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 800.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let stick_length: f32 = 30.;
    let (origin_x, origin_y) = (-450., 350.);
    let (points_x_count, points_y_count) = (20, 15);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn_bundle(VerletPointSpriteBundle {
                verlet_point: VerletPoint2::new(Vec2::new(
                    origin_x + (30. * i as f32),
                    origin_y + (-30. * j as f32),
                )),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite::new(Vec2::splat(10.)),
                    material: material.clone(),
                    ..Default::default()
                },
                ..Default::default()
            });
            cmd.insert(Name::new(format!("Point {}", i)));
            if j == 0 {
                cmd.insert(VerletLocked {}).insert(fixed_material.clone());
            }
            entities.push(cmd.id());
        }
    }
    for (i, entity) in entities.iter().enumerate() {
        let above = i.checked_sub(points_x_count);
        let left = if i % points_x_count == 0 {
            None
        } else {
            Some(i - 1)
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
        commands.spawn().insert(VerletStick {
            point_a_entity: entity,
            point_b_entity: *other_entity,
            length,
        });
    }
}
