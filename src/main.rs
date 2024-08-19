use bevy::prelude::*;

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
        .add_systems(Update, (update_position, print_info))
        .run();
}

fn step_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship_J.png"),
            ..default()
        },
        Velocity { value: 300.0 },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ship_L.png"),
            transform: Transform::from_xyz(-200.0, 50.0, 0.0),
            ..default()
        },
        Velocity { value: 300.0 },
    ));
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity2d) in query.iter_mut() {
        let dir = transform.up();
        transform.translation += dir * velocity2d.value * time.delta_seconds();
        transform.rotate_z(1.75 * time.delta_seconds());
    }
}

fn print_info(query: Query<(Entity, &Transform)>) {
    for (entity, velocity) in query.iter() {
        println!("{:?} {:?}", entity, velocity);
    }
}
