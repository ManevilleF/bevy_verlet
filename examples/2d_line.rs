use bevy::prelude::*;
use bevy_verlet::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D line".to_string(),
                resolution: (1000., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VerletPlugin::default())
        .add_systems(Startup, (setup_camera, setup_free_line, setup_fixed_line))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_free_line(mut commands: Commands) {
    let stick_length: f32 = 50.;
    let points_count = 10;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(10.)),
                ..default()
            },
            Transform::from_xyz(50.0 * i as f32, 300.0, 0.),
            VerletPoint::new(0.1),
            Name::new(format!("Point {}", i)),
        ));
        if previous_entity.is_none() {
            cmd.insert(VerletLocked);
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

fn setup_fixed_line(mut commands: Commands) {
    let stick_length: f32 = 35.;
    let points_count = 20;
    let start_pos = -450.;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(10.)),
                ..default()
            },
            Transform::from_xyz(start_pos + 30.0 * i as f32, 0.0, 0.),
            VerletPoint::default(),
            Name::new(format!("Point {}", i)),
        ));
        if previous_entity.is_none() || i == points_count {
            cmd.insert(VerletLocked);
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
