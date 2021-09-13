use bevy::prelude::*;
use bevy_verlet::{BevyVerletPlugin, VerletLocked, VerletPointSpriteBundle, VerletStick};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyVerletPlugin::default())
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 500.,
            ..Default::default()
        })
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_free_line.system())
        .add_startup_system(setup_fixed_line.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_free_line(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let stick_length: f32 = 50.;
    let points_count = 10;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn_bundle(verlet_bundle(
            material.clone(),
            Vec2::new(50. * i as f32, 300.),
        ));
        cmd.insert(Name::new(format!("Point {}", i)));
        if previous_entity.is_none() {
            cmd.insert(VerletLocked {}).insert(fixed_material.clone());
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands
                .spawn()
                .insert(VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                })
                .insert(Name::new(format!("Stick {}", i)));
        }
        previous_entity = Some(entity);
    }
}

fn setup_fixed_line(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material = materials.add(Color::WHITE.into());
    let fixed_material = materials.add(Color::RED.into());
    let stick_length: f32 = 35.;
    let points_count = 20;
    let start_pos = -450.;
    let mut previous_entity = None;
    for i in 0..=points_count {
        let mut cmd = commands.spawn_bundle(verlet_bundle(
            material.clone(),
            Vec2::new(start_pos + 30. * i as f32, 0.),
        ));
        cmd.insert(Name::new(format!("Point {}", i)));
        if previous_entity.is_none() || i == points_count {
            cmd.insert(VerletLocked {}).insert(fixed_material.clone());
        }
        let entity = cmd.id();
        if let Some(e) = previous_entity {
            commands
                .spawn()
                .insert(VerletStick {
                    point_a_entity: e,
                    point_b_entity: entity,
                    length: stick_length,
                })
                .insert(Name::new(format!("Stick {}", i)));
        }
        previous_entity = Some(entity);
    }
}

fn verlet_bundle(material: Handle<ColorMaterial>, pos: Vec2) -> VerletPointSpriteBundle {
    VerletPointSpriteBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite::new(Vec2::splat(10.)),
            material,
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            ..Default::default()
        },
        ..Default::default()
    }
}
