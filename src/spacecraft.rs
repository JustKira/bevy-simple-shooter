use avian2d::prelude::*;
use bevy::prelude::*;
pub struct SpaceCraftPlugin;

#[derive(Component)]
pub struct SpaceCraft {}

#[derive(Component)]
pub struct MaxSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct MaxRotationSpeed {
    pub value: f32,
}

impl Plugin for SpaceCraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spacecraft)
            .add_systems(Update, (spacecraft_movement, spacecraft_rotation));
    }
}

fn spawn_spacecraft(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        RigidBody::Dynamic,
        SpriteBundle {
            texture: asset_server.load("ship_B.png"),
            ..default()
        },
        GravityScale(0.0),
        Collider::circle(10.0),
        MaxSpeed { value: 100.0 },
        MaxRotationSpeed { value: 5.0 },
        SpaceCraft {},
    ));
}

fn spacecraft_movement(
    mut query: Query<(&mut LinearVelocity, &Transform, &MaxSpeed), With<SpaceCraft>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut linear_velocity, transform, max_speed) in query.iter_mut() {}
}
fn spacecraft_rotation(
    mut query: Query<(&mut AngularVelocity, &MaxRotationSpeed), With<SpaceCraft>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut angular_velocity, max_rotation_speed) in query.iter_mut() {}
}
