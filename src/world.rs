use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_world, spawn_objects));
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 100.0,
                color: Color::Rgba {
                    red: 1.,
                    green: 0.78,
                    blue: 0.,
                    alpha: 1.,
                },
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Sun"),
    );

    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10., 0.001, 10.),
        Name::new("Floor"),
    );

    commands.spawn(light);
    commands.spawn(floor);
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cube = |size: f32,
                           color: Color,
                           xyz: (f32, f32, f32),
                           name: String|
     -> (PbrBundle, RigidBody, Collider, Name) {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                material: materials.add(color.into()),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(size / 2., size / 2., size / 2.),
            Name::new(name),
        )
    };

    commands.spawn(create_cube(
        4.0,
        Color::BLUE,
        (-5.0, 2.0, 5.0),
        "Blue Cube".to_string(),
    ));

    commands.spawn(create_cube(
        2.0,
        Color::RED,
        (6.0, 1.0, -6.0),
        "Red Cube".to_string(),
    ));

    commands.spawn(create_cube(
        0.5,
        Color::PURPLE,
        (2.0, 0.25, 1.0),
        "Ominous Cube".to_string(),
    ));
}
