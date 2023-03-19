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
        .add_plugin(VerletPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_free_line)
        .add_startup_system(setup_fixed_line)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_free_line(mut commands: Commands) {
    let stick_length: f32 = 50.;
    let points_count = 10;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn((
            sprite_bundle(Color::WHITE, Vec2::new(50. * i as f32, 300.)),
            VerletPoint::default(),
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
            sprite_bundle(Color::WHITE, Vec2::new(start_pos + 30. * i as f32, 0.)),
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

fn sprite_bundle(color: Color, pos: Vec2) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::splat(10.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(pos.x, pos.y, 0.),
        ..Default::default()
    }
}
