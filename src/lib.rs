use godot::prelude::*;

// Declare the agent module
mod agent;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


pub fn hello() {
    println!("Hello from the library!");
}