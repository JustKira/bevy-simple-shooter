use bevy::prelude::*;

pub struct SpaceCraftPlugin;

impl Plugin for SpaceCraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spacecraft);
    }
}

fn spawn_spacecraft(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning SpaceCraft");

    commands.spawn(SpriteBundle {
        texture: asset_server.load("ship_B.png"),
        ..default()
    });
}
