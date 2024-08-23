use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
pub struct SpaceCraftPlugin;

#[derive(Event)]
pub enum SpaceCraftVerticalAction {
    VerticalMove(Scalar),
}

#[derive(Event)]
pub enum SpaceCraftHorizontalAction {
    HorizontalMove(Scalar),
}

#[derive(Component)]
pub struct SpaceCraft {}

// #[derive(Component)]
// pub struct SpaceCraftHorizontalMovement {}

// #[derive(Component)]
// pub struct SpaceCraftLinearAcceleration(Scalar);

// #[derive(Component)]
// pub struct SpaceCraftAngularAcceleration(Scalar);
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
        app.add_event::<SpaceCraftVerticalAction>()
            .add_event::<SpaceCraftHorizontalAction>()
            .add_systems(Startup, spawn_spacecraft)
            .add_systems(
                Update,
                (
                    spacecraft_vertical_input,
                    spacecraft_horizontal_input,
                    spacecraft_local_vertical_movement,
                    spacecraft_rotation,
                )
                    .chain(),
            );
    }
}

fn spacecraft_vertical_input(
    mut event_writer: EventWriter<SpaceCraftVerticalAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up: bool = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

    let vertical = up as i8 - down as i8;
    let vertical_direction = vertical as Scalar;

    if vertical_direction != 0.0 {
        event_writer.send(SpaceCraftVerticalAction::VerticalMove(vertical_direction));
    }
}

fn spacecraft_horizontal_input(
    mut event_writer: EventWriter<SpaceCraftHorizontalAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = left as i8 - right as i8;
    let horizontal_direction = horizontal as Scalar;

    if horizontal_direction != 0.0 {
        event_writer.send(SpaceCraftHorizontalAction::HorizontalMove(
            horizontal_direction,
        ));
    }
}

fn spawn_spacecraft(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        RigidBody::Kinematic,
        SpriteBundle {
            texture: asset_server.load("ship_B.png"),
            ..default()
        },
        GravityScale(0.0),
        Collider::circle(10.0),
        MaxSpeed { value: 100.0 },
        MaxRotationSpeed { value: 5.0 },
        LinearDamping::from(0.9),
        AngularDamping::from(0.9),
        SpaceCraft {},
    ));
}

fn spacecraft_local_vertical_movement(
    mut query: Query<(&mut LinearVelocity, &Transform, &MaxSpeed), With<SpaceCraft>>,
    mut spacecraft_action_reader: EventReader<SpaceCraftVerticalAction>,
    time: Res<Time>,
) {
    for event in spacecraft_action_reader.read() {
        for (mut linear_velocity, transform, max_speed) in query.iter_mut() {
            match event {
                SpaceCraftVerticalAction::VerticalMove(direction) => {
                    let spacecraft_direction = transform.up().normalize();
                    let spacecraft_direction2d =
                        Vec2::new(spacecraft_direction.x, spacecraft_direction.y);

                    linear_velocity.0 +=
                        spacecraft_direction2d * *direction * 20.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn spacecraft_rotation(
    mut query: Query<(&mut AngularVelocity, &Transform, &MaxRotationSpeed), With<SpaceCraft>>,
    mut spacecraft_action_reader: EventReader<SpaceCraftHorizontalAction>,
    time: Res<Time>,
) {
    for event in spacecraft_action_reader.read() {
        for (mut angular_velocity, transform, max_rotation_speed) in query.iter_mut() {
            match event {
                SpaceCraftHorizontalAction::HorizontalMove(direction) => {
                    angular_velocity.0 +=
                        *direction * max_rotation_speed.value * time.delta_seconds();
                }
            }
        }
    }
}
