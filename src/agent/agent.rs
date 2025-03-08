use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::agent::move_and_build_behaviour::MoveAndBuildBehaviour;
use crate::building::{home_building_config, Building, Field, FieldState, IBuilding};

use super::free_space_manager::FreeSpaceManager;
use super::move_behaviour::{MoveBehaviour, Result};
use crate::resources::inventory::{Inventory, InventoryResource};
#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    HomeBuilding,
    FieldBuilding,
    FieldRemoving,
    ReturningToHome,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Agent {
    state: State,
    base: Base<Sprite2D>,
    home_build_behaviour: MoveAndBuildBehaviour<Building>,
    field_build_behaviour: MoveAndBuildBehaviour<Field>,
    move_behaviour: MoveBehaviour,
    fields: Vec<Gd<Field>>,
    home: Option<Gd<Building>>,
    max_field_count: usize,
    home_building_radius: f32,
    field_building_radius: f32,
    removing_field: Option<Gd<Field>>,
    inventory: Inventory,
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
            move_behaviour: MoveBehaviour::new(),
            fields: Vec::new(),
            home: None,
            max_field_count: 3,
            home_building_radius: 600.0,
            field_building_radius: 100.0,
            removing_field: None,
            inventory: Inventory::new(),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        loop {
            match self.state {
                State::Idle => {
                    let agent_name = self.base().get_name().to_string();
                    self.home_build_behaviour.set_agent_name(agent_name.clone());
                    self.field_build_behaviour.set_agent_name(agent_name);
                    if self.home.is_none() {
                        self.start_home_building();
                    } else if self.is_any_field_completed() {
                        self.start_field_removing();
                    } else if self.fields.len() < self.max_field_count {
                        self.start_field_building();
                    } else {
                        return;
                    }
                }
                State::HomeBuilding => {
                    let result = self.move_and_build_home(delta);
                    if result == Result::Running {
                        return;
                    }
                    if result == Result::Success {
                        godot_print!("Agent {} Home build complete", self.base().get_name());
                        self.state = State::Idle;
                    }
                }
                State::FieldBuilding => {
                    let result = self.move_and_build_field(delta);
                    if result == Result::Running {
                        return;
                    }
                    if result == Result::Success {
                        godot_print!("Agent {} Field build complete", self.base().get_name());
                        self.state = State::Idle;
                    }
                }
                State::FieldRemoving => {
                    let result = self.move_and_remove_field(delta);
                    if result == Result::Running {
                        return;
                    }
                    if result == Result::Success {
                        godot_print!("Agent {} Field removing complete", self.base().get_name());
                        self.finish_field_removing();
                        self.start_returning_to_home();
                    }
                }
                State::ReturningToHome => {
                    let result = self.move_to_home(delta);
                    if result == Result::Running {
                        return;
                    }
                    self.finish_returning_to_home();
                }
            }
        }
    }
}

impl Agent {
    fn start_home_building(&mut self) {
        godot_print!("Agent {} Starting home build", self.base().get_name());
        let build_position = self.calculate_free_space_position(self.home_building_radius);
        let home = Building::from_config_and_position(home_building_config(), build_position);
        self.add_building_to_parent(&home);
        self.home_build_behaviour.start_construction(home.clone(), self.base().get_position());
        self.home = Some(home);
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
        godot_print!("Agent {} Starting field build", self.base().get_name());
        let build_position = self.calculate_free_space_position(self.field_building_radius);
        let field = Field::from_position(build_position);
        self.add_building_to_parent(&field);
        self.field_build_behaviour.start_construction(field.clone(), self.base().get_position());
        self.fields.push(field);
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
        let build_position =
            free_space_manager.bind().find_random_free_position_near(self.base().get_position(), radius);
        free_space_manager.bind_mut().add_occupied_position(build_position);
        build_position
    }

    fn add_building_to_parent<T: IBuilding + Inherits<Node>>(&self, building: &Gd<T>) {
        let node = building.clone().upcast::<Node>();
        let mut parent = self.base().get_parent().expect("Parent not found");
        parent.add_child(&node);
    }

    fn is_any_field_completed(&self) -> bool {
        self.fields.iter().any(|field| field.bind().state == FieldState::Grown)
    }

    fn start_field_removing(&mut self) {
        godot_print!("Agent {} Starting field removing", self.base().get_name());
        let field = self.fields.iter().find(|field| field.bind().state == FieldState::Grown).unwrap();
        self.removing_field = Some(field.clone());
        self.field_build_behaviour.start_deconstruction(field.clone(), self.base().get_position());
        self.state = State::FieldRemoving;
    }

    fn move_and_remove_field(&mut self, delta: f64) -> Result {
        let (result, next_position) = self.field_build_behaviour.build(delta);
        if let Some(next_position) = next_position {
            self.base_mut().set_position(next_position);
        }
        result
    }

    fn finish_field_removing(&mut self) {
        godot_print!("Agent {} Field removing complete", self.base().get_name());
        let mut free_space_manager = FreeSpaceManager::singleton();
        let field_instance_id = self.removing_field.as_ref().unwrap().instance_id();
        self.fields.retain(|field| field.instance_id() != field_instance_id);
        free_space_manager
            .bind_mut()
            .remove_occupied_position(self.removing_field.as_ref().unwrap().bind().base().get_position());
        self.removing_field.as_mut().unwrap().bind_mut().base_mut().queue_free();
        self.removing_field = None;
        self.inventory.add(InventoryResource::Wheat, 1);
        self.state = State::Idle;
    }

    fn start_returning_to_home(&mut self) {
        if self.home.is_none() {
            self.state = State::Idle;
            return;
        }
        godot_print!("Agent {} Starting returning to home", self.base().get_name());
        self.move_behaviour
            .start_moving(self.base().get_position(), self.home.as_ref().unwrap().bind().base().get_position());
        self.state = State::ReturningToHome;
    }

    fn move_to_home(&mut self, delta: f64) -> Result {
        let (result, next_position) = self.move_behaviour.move_agent(delta);
        self.base_mut().set_position(next_position);
        result
    }

    fn finish_returning_to_home(&mut self) {
        godot_print!("Agent {} Returning to home complete", self.base().get_name());
        self.home.as_mut().unwrap().bind_mut().inventory.move_full_inventory_from(&mut self.inventory);
        self.state = State::Idle;
    }
}
