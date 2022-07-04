use bevy::prelude::*;
use bevy_verlet::{
    VerletConfig, VerletLocked, VerletPlugin, VerletPoint, VerletStick, VerletStickMaxTension,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "2D Cloth cutter".to_string(),
            width: 1400.,
            height: 900.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(VerletPlugin::default())
        .insert_resource(VerletConfig {
            parallel_processing_batch_size: Some(500),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(cut_sticks)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let stick_length: f32 = 11.;
    let (origin_x, origin_y) = (-600., 420.);
    let (points_x_count, points_y_count) = (121, 60);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn();
            cmd.insert(Transform::from_xyz(
                origin_x + (10. * i as f32),
                origin_y + (-10. * j as f32),
                0.,
            ))
            .insert(GlobalTransform::default())
            .insert(VerletPoint::default())
            .insert(Name::new(format!("Point {}", i)));
            if j == 0 && i % 2 == 0 {
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
        commands
            .spawn()
            .insert(VerletStick {
                point_a_entity: entity,
                point_b_entity: *other_entity,
                length,
            })
            .insert(VerletStickMaxTension(5.));
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
    let l = 20.;
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
            commands.entity(entity).despawn_recursive();
        }
    }
}
