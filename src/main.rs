use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
struct SpaceCraftStats {
    max_speed: f32,
    active_speed: f32,
    min_speed: f32,
    rot_speed: f32,
    rot_accel: f32,
    craft_accel: f32,
}

#[derive(Component, Debug)]
struct PlayerCamera;

#[derive(Component, Debug)]
struct Enemy;

#[derive(Component, Debug)]
struct SpaceCraft {
    velocity: f32,
    rot_velocity: f32,
}

#[derive(Component, Debug)]
struct Bullet {
    velocity: f32,
    damage: f32,
    lifetime: f32,
    time_alive: f32,
}

fn main() {
    App::new()
        //This is the default plugin that comes with bevy which includes the renderer and ui
        .add_plugins(DefaultPlugins)
        // Adding the system to the app
        .add_systems(Startup, step_up)
        .add_systems(
            Update,
            (player_movement, camera_movement, print_craft_stats).chain(),
        )
        .add_systems(Update, bullet_movement)
        .run();
}

fn step_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                // don't forget to set `near` and `far`
                near: -1000.0,
                far: 5000.0,
                scale: 2.0,
                // ... any other settings you want to change ...
                ..default()
            },
            ..default()
        },
        PlayerCamera,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("meteor_large.png"),
            transform: Transform::from_xyz(75.0, 50.0, 0.0),
            ..default()
        },
        Enemy,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship_H.png"),
            transform: Transform::from_xyz(250.0, 0.0, 0.0),
            ..default()
        },
        SpaceCraft {
            velocity: 0.0,
            rot_velocity: 0.0,
        },
        Player {},
        SpaceCraftStats {
            max_speed: 700.0,
            active_speed: 300.0,
            min_speed: 150.0,
            rot_speed: 5.0,
            rot_accel: 0.25,
            craft_accel: 5.5,
        },
    ));
}

fn bullet_movement(
    mut query: Query<(Entity, &mut Transform, &mut Bullet)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut bullet) in query.iter_mut() {
        let dir = transform.up().normalize();
        bullet.time_alive += time.delta_seconds();
        if bullet.time_alive > bullet.lifetime {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += dir * bullet.velocity * time.delta_seconds();
    }
}

fn player_movement(
    mut query: Query<(&mut Transform, &mut SpaceCraft, &SpaceCraftStats), With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut space_craft, space_craft_stats) in query.iter_mut() {
        let dir = transform.up().normalize();

        let current_velocity = space_craft.velocity;

        if keyboard_input.pressed(KeyCode::KeyW) {
            space_craft.velocity += space_craft_stats.craft_accel;
            if space_craft.velocity > space_craft_stats.max_speed {
                space_craft.velocity = space_craft_stats.max_speed;
            }
            transform.translation += dir * space_craft.velocity * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            space_craft.velocity -= space_craft_stats.craft_accel;
            if space_craft.velocity < space_craft_stats.min_speed {
                space_craft.velocity = space_craft_stats.min_speed;
            }
            transform.translation += dir * space_craft.velocity * time.delta_seconds();
        } else {
            let sign = space_craft_stats.active_speed - current_velocity;
            space_craft.velocity += sign.signum() * space_craft_stats.craft_accel;

            if space_craft.velocity > space_craft_stats.active_speed {
                space_craft.velocity -= space_craft_stats.craft_accel;
                if space_craft.velocity < space_craft_stats.active_speed {
                    space_craft.velocity = space_craft_stats.active_speed;
                }
            } else if space_craft.velocity < space_craft_stats.active_speed {
                space_craft.velocity += space_craft_stats.craft_accel;
                if space_craft.velocity > space_craft_stats.active_speed {
                    space_craft.velocity = space_craft_stats.active_speed;
                }
            }

            transform.translation += dir * space_craft.velocity * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            space_craft.rot_velocity += space_craft_stats.rot_accel;
            if space_craft.rot_velocity > space_craft_stats.rot_speed {
                space_craft.rot_velocity = space_craft_stats.rot_speed;
            }
            transform.rotate_z(space_craft.rot_velocity * time.delta_seconds());
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            space_craft.rot_velocity -= space_craft_stats.rot_accel;
            if space_craft.rot_velocity < -space_craft_stats.rot_speed {
                space_craft.rot_velocity = -space_craft_stats.rot_speed;
            }
            transform.rotate_z(space_craft.rot_velocity * time.delta_seconds());
        } else {
            if space_craft.rot_velocity > 0.0 {
                space_craft.rot_velocity -= space_craft_stats.rot_accel;
                if space_craft.rot_velocity < 0.0 {
                    space_craft.rot_velocity = 0.0;
                }
            } else if space_craft.rot_velocity < 0.0 {
                space_craft.rot_velocity += space_craft_stats.rot_accel;
                if space_craft.rot_velocity > 0.0 {
                    space_craft.rot_velocity = 0.0;
                }
            }
            transform.rotate_z(space_craft.rot_velocity * time.delta_seconds());
        }

        if keyboard_input.pressed(KeyCode::Space) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("bullet.png"),
                    transform: transform.clone(),
                    ..default()
                },
                Bullet {
                    velocity: 1500.0,
                    damage: 10.0,
                    time_alive: 0.0,
                    lifetime: 5.0,
                },
            ));
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

fn print_craft_stats(query: Query<&SpaceCraft>) {
    for space_craft in query.iter() {
        println!(
            "V:{:?} RV:{:?}",
            space_craft.velocity, space_craft.rot_velocity
        );
    }
}

fn camera_movement(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    time: Res<Time>,
) {
    let smoothing_factor = 5.5; // Adjust this value for faster or slower smoothing

    for mut transform in &mut camera {
        for player_transform in &player {
            // Interpolate between the current camera position and the player's position
            transform.translation.x = transform.translation.x.lerp(
                player_transform.translation.x,
                smoothing_factor * time.delta_seconds(),
            );
            transform.translation.y = transform.translation.y.lerp(
                player_transform.translation.y,
                smoothing_factor * time.delta_seconds(),
            );
        }
    }
}
