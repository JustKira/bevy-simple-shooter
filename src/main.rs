mod spacecraft;
mod world;

use avian2d::prelude::*;
use bevy::prelude::*;
use spacecraft::SpaceCraftPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        //This is the default plugin that comes with bevy which includes the renderer and ui
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(WorldPlugin)
        .add_plugins(SpaceCraftPlugin)
        .add_systems(Startup, hello_world)
        // Adding the system to the app
        .run();
}

fn hello_world() {
    println!("Hello, World!");
}
