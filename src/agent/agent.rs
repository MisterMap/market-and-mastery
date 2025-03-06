use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

use crate::agent::MoveAndBuildBehaviour;
use crate::building::{home_building_config, Building, Field, IBuilding};

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
    home_build_behaviour: MoveAndBuildBehaviour<Building>,
    field_build_behaviour: MoveAndBuildBehaviour<Field>,
    field_count: u32,
    max_field_count: u32,
    home_building_radius: f32,
    field_building_radius: f32,
}

#[godot_api]
impl ISprite2D for Agent {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");
        Self {
            state: State::Idle,
            base,
            home_build_behaviour: MoveAndBuildBehaviour::new(),
            field_build_behaviour: MoveAndBuildBehaviour::new(),
            field_count: 0,
            max_field_count: 3,
            home_building_radius: 600.0,
            field_building_radius: 100.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.home_build_behaviour.set_agent_name(self.base().get_name().to_string());
            self.start_home_building();
        }
        if self.state == State::HomeBuilding {
            let result = self.move_and_build_home(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Build complete", self.base().get_name());
                self.start_field_building();
            }
        }
        if self.state == State::FieldBuilding {
            let result = self.move_and_build_field(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Build complete", self.base().get_name());
                self.field_count += 1;
                self.start_field_building();
            }
        }
    }
}

impl Agent {
    fn start_home_building(&mut self) {
        godot_print!("Agent {} Starting home build", self.base().get_name());
        let build_position = self.calculate_free_space_position(self.home_building_radius);
        let building = Building::from_config_and_position(home_building_config(), build_position);
        self.add_building_to_parent(&building);
        self.home_build_behaviour.start(building, self.base().get_position());
        self.state = State::HomeBuilding;
    }

    fn move_and_build_home(&mut self, delta: f64) -> Result {
        let (result, next_position) = self.home_build_behaviour.build(delta);
        if let Some(next_position) = next_position {
            self.base_mut().set_position(next_position);
        }
        result
    }

    fn start_field_building(&mut self) {
        if self.field_count >= self.max_field_count {
            self.state = State::Finished;
            return;
        }
        let build_position = self.calculate_free_space_position(self.field_building_radius);
        let building = Field::from_position(build_position);
        self.add_building_to_parent(&building);
        self.field_build_behaviour.start(building, self.base().get_position());
        self.state = State::FieldBuilding;
    }

    fn move_and_build_field(&mut self, delta: f64) -> Result {
        let (result, next_position) = self.field_build_behaviour.build(delta);
        if let Some(next_position) = next_position {
            self.base_mut().set_position(next_position);
        }
        result
    }

    fn calculate_free_space_position(&self, radius: f32) -> Vector2 {
        let mut free_space_manager = FreeSpaceManager::singleton();
        let build_position = free_space_manager.bind().find_random_free_position_near(self.base().get_position(), radius);
        free_space_manager.bind_mut().add_occupied_position(build_position);
        build_position
    }

    fn add_building_to_parent<T: IBuilding + Inherits<Node>>(&self, building: &Gd<T>) {
        let node = building.clone().upcast::<Node>();
        let mut parent = self.base().get_parent().expect("Parent not found");
        parent.add_child(&node);
    }
}