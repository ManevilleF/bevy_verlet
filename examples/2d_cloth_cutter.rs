use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};
use bevy_verlet::prelude::*;

const MOUSE_RADIUS: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Cloth cutter".to_string(),
                resolution: (1400., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VerletPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (drag_points, cut_sticks))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    let stick_length: f32 = 11.;
    let (origin_x, origin_y) = (-690., 420.);
    let (points_x_count, points_y_count) = (139, 80);
    let mut entities = Vec::new();
    for j in 0..points_y_count {
        for i in 0..points_x_count {
            let mut cmd = commands.spawn((
                Transform::from_xyz(
                    origin_x + (10. * i as f32),
                    origin_y + (-10. * j as f32),
                    0.,
                ),
                VerletPoint::new(0.1),
                Name::new(format!("Point {}", i)),
            ));
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
        commands.spawn((
            VerletStick {
                point_a_entity: entity,
                point_b_entity: *other_entity,
                length,
            },
            VerletStickMaxTension(3.),
        ));
    }
}

fn mouse_coords(window: &Window, position: Vec2) -> Vec2 {
    Vec2::new(
        position.x - window.width() / 2.0,
        window.height() / 2.0 - position.y,
    )
}

fn cut_sticks(
    mut commands: Commands,
    points: Query<&Transform, With<VerletPoint>>,
    sticks: Query<(Entity, &VerletStick)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }
    let window = windows.single();
    let p = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    for (entity, stick) in sticks.iter() {
        let [a, b] = points
            .get_many(stick.entities())
            .map(|v| v.map(|t| t.translation.xy()))
            .unwrap();
        let distance_a = p.distance(a);
        let distance_b = p.distance(b);
        if distance_a > 0.
            && distance_a <= MOUSE_RADIUS
            && distance_b > 0.
            && distance_b <= MOUSE_RADIUS
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn drag_points(
    mut points: Query<(Entity, &mut Transform), With<VerletPoint>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut dragged: Local<Vec<Entity>>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        *dragged = vec![];
        return;
    }
    let window = windows.single();
    let p = match window.cursor_position() {
        None => return,
        Some(p) => mouse_coords(window, p),
    };
    if dragged.is_empty() {
        *dragged = points
            .iter()
            .filter_map(|(e, tr)| {
                let point = tr.translation.xy();
                let dist = point.distance(p);
                (dist <= MOUSE_RADIUS).then_some(e)
            })
            .collect();
        return;
    };
    let mut iter = points.iter_many_mut(&*dragged);
    while let Some((_, mut tr)) = iter.fetch_next() {
        tr.translation.x = p.x;
        tr.translation.y = p.y;
    }
}
