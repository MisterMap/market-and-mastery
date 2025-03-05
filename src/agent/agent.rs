use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

use crate::agent::MoveAndBuildBehaviour;
use crate::building::{field_building_config, home_building_config, Building, BuildingConfig};

use super::free_space_manager::FreeSpaceManager;
use super::move_behaviour::Result;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    HomeBuilding,
    FieldBuilding,
    Finished,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Agent {
    state: State,
    base: Base<Sprite2D>,
    build_behaviour: MoveAndBuildBehaviour,
    field_count: u32,
    max_field_count: u32,
    home_building_radius: f32,
    field_building_radius: f32,
}

#[godot_api]
impl ISprite2D for Agent {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");
        let mut agent = Self {
            state: State::Idle,
            base,
            build_behaviour: MoveAndBuildBehaviour::new(),
            field_count: 0,
            max_field_count: 3,
            home_building_radius: 600.0,
            field_building_radius: 100.0,
        };
        agent.build_behaviour.set_agent_name(agent.base().get_name().to_string());
        agent
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.start_home_building();
        }
        if self.state == State::HomeBuilding {
            let result = self.move_and_build(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Build complete", self.base().get_name());
                self.start_field_building();
            }
        }
        if self.state == State::FieldBuilding {
            let result = self.move_and_build(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Build complete", self.base().get_name());
                self.start_field_building();
            }
        }
    }
}

impl Agent {
    fn start_building(&mut self, building_config: BuildingConfig, radius: f32) {
        godot_print!("Agent {} Starting build {}", self.base().get_name(), building_config.building_name);
        let mut parent = self.base().get_parent().expect("Parent not found");
        let mut free_space_manager = FreeSpaceManager::singleton();
        let build_position = free_space_manager.bind().find_random_free_position_near(self.base().get_position(), radius);
        let building = Building::from_config_and_position(building_config, build_position);
        free_space_manager.bind_mut().add_occupied_position(build_position);
        let sprite_node = building.clone().upcast::<Node>();
        parent.add_child(&sprite_node);
        self.build_behaviour.start(building, self.base().get_position());
    }

    fn start_home_building(&mut self) {
        self.start_building(home_building_config(), self.home_building_radius);
        self.state = State::HomeBuilding;
    }

    fn start_field_building(&mut self) {
        if self.field_count >= self.max_field_count {
            self.state = State::Finished;
            return;
        }
        self.start_building(field_building_config(), self.field_building_radius);
        self.field_count += 1;
        self.state = State::FieldBuilding;
    }

    fn move_and_build(&mut self, delta: f64) -> Result {
        let (result, next_position) = self.build_behaviour.build(delta);
        if let Some(next_position) = next_position {
            self.base_mut().set_position(next_position);
        }
        result
    }
}