use bevy::{input::keyboard::KeyboardInput, prelude::*};

// #[derive(Component, Debug)]
// This is used to tell bevy that this is a component
// Components are used to store data that can be attached to entities
// Entities are used to represent objects in the game world
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Rotation {
    angle: f32,
}

fn main() {
    App::new()
        //This is the default plugin that comes with bevy which includes the renderer and ui
        .add_plugins(DefaultPlugins)
        // Adding the system to the app
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_info, accelerate_ship))
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    // We are using commands.spawn() to create a new entity
    // We are adding 2 Components to the entity Position and Velocity
    // Which makes up the Entity
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 0.0 }, Rotation { angle: 0.0 }));
}

fn accelerate_ship(KeyboardInput: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Velocity, &Rotation)>) {

    let acceleration: f32 = 0.1;


    for (mut velocity, rotation) in query.iter_mut() {
        if KeyboardInput.pressed(KeyCode::KeyW){
            velocity.x += acceleration * rotation.angle.cos();
            velocity.y += acceleration * rotation.angle.sin();
        }
    
        if KeyboardInput.pressed(KeyCode::KeyS) {
           
            let current_speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
            let new_speed = (current_speed - acceleration).max(0.0);

            if current_speed > 0.0 {
                let scale = new_speed / current_speed;
                velocity.x *= scale;
                velocity.y *= scale;
            }
         }
    }
}

// Query is like database we are fetching the Entities that Has VELOCITY & POSITION
// We didn't add &mut because we are not modifying the velocity we are just reading value
// But we added &mut because we are modifying the position by using readonly velocity
fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    // We are using query.iter_mut() to get the iterator of the query
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

// We are fetching the Entities that Has POSITION only
// We didn't add &mut because we are not modifying the position we are just reading value
fn print_info(query: Query<(Entity, &Position, &Rotation, &Velocity)>) {
    // We are using query.iter() to get the iterator of the query
    // Note we are not using query.iter_mut() because we are not modifying the position
    for (entity, position, rotation, velocity) in query.iter() {
        println!("{:?} {:?} {:?} {:?}", entity, position, rotation, velocity);
    }
}
