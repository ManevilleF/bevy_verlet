use bevy::prelude::*;
use bevy_verlet::{BevyVerletPlugin, VerletLocked, VerletPoint, VerletStick};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyVerletPlugin::default())
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 1000.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(cut_sticks.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let stick_length: f32 = 31.;
    let (origin_x, origin_y) = (-450., 350.);
    let (points_x_count, points_y_count) = (31, 20);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::splat(10.)),
                material: material.clone(),
                transform: Transform::from_xyz(
                    origin_x + (30. * i as f32),
                    origin_y + (-30. * j as f32),
                    0.,
                ),
                ..Default::default()
            });
            cmd.insert(VerletPoint::default())
                .insert(Name::new(format!("Point {}", i)));
            if j == 0 && i % 2 == 0 {
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

fn mouse_coords(window: &Window, position: Vec2) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    position - window_size / 2.
}

fn cut_sticks(
    mut commands: Commands,
    points: Query<&Transform, With<VerletPoint>>,
    sticks: Query<(Entity, &VerletStick)>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    let window = windows.get_primary().unwrap();
    let p = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    let l = 40.;
    for (entity, stick) in sticks.iter() {
        let point_a = points.get(stick.point_a_entity).unwrap();
        let point_b = points.get(stick.point_b_entity).unwrap();
        let (a, b) = (
            Vec2::new(point_a.translation.x, point_a.translation.y),
            Vec2::new(point_b.translation.x, point_b.translation.y),
        );
        let distance_a = p.distance(a);
        let distance_b = p.distance(b);
        if distance_a > 0. && distance_a <= l && distance_b > 0. && distance_b <= l {
            commands.entity(entity).despawn();
        }
    }
}
