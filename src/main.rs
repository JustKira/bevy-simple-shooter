use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct PlayerCamera;

#[derive(Component, Debug)]
struct Enemy;

#[derive(Component, Debug)]
struct Velocity {
    value: f32,
}

fn main() {
    App::new()
        //This is the default plugin that comes with bevy which includes the renderer and ui
        .add_plugins(DefaultPlugins)
        // Adding the system to the app
        .add_systems(Startup, step_up)
        .add_systems(Update, (player_movement, camera_movement).chain())
        .run();
}

fn step_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship_J.png"),
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            ..default()
        },
        Velocity { value: 200.0 },
        Player,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_A.png"),
            transform: Transform::from_xyz(75.0, 50.0, 0.0),
            ..default()
        },
        Velocity { value: 150.0 },
        Enemy,
    ));
}

fn player_movement(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, velocity2d) in query.iter_mut() {
        let dir = transform.up();

        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation += dir * velocity2d.value * time.delta_seconds() * 2.15;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += dir * velocity2d.value * time.delta_seconds() * 0.55;
        } else {
            transform.translation += dir * velocity2d.value * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_z(4.0 * time.delta_seconds());
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_z(-4.0 * time.delta_seconds());
        }
    }
}

// fn enemy_movement(mut query: Query<(&mut Transform, &Velocity), With<Enemy>>, time: Res<Time>) {
//     for (mut transform, velocity2d) in query.iter_mut() {
//         let dir = transform.up();
//         transform.translation += dir * velocity2d.value * time.delta_seconds();
//         transform.rotate_z(1.5 * time.delta_seconds());
//     }
// }

fn camera_movement(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    for mut transform in &mut camera {
        for player_transform in &player {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}
