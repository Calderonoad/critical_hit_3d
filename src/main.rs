use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_world, spawn_player))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(camera);
    commands.spawn(light);
    commands.spawn(floor);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(player);
}
