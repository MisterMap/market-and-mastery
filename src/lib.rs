use godot::prelude::*;
use godot::classes::Engine;
use godot::classes::Object;
pub mod agent;
pub mod behaviour;
pub mod building;
pub mod resources;

use behaviour::free_space_manager::FreeSpaceManager;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // Register the FreeSpaceManager singleton
            godot_print!("Registering FreeSpaceManager singleton");
            let singleton = FreeSpaceManager::new_alloc();
            Engine::singleton()
                .register_singleton(
                    &StringName::from("FreeSpaceManager"),
                    &singleton.upcast::<Object>()
                );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Remove the singleton when the library is unloaded
            godot_print!("Unregistering FreeSpaceManager singleton");
            Engine::singleton()
                .unregister_singleton(&StringName::from("FreeSpaceManager"));
        }
    }
}

pub fn hello() {
    println!("Hello from the library!");
}