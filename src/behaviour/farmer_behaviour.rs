use super::free_space_manager::FreeSpaceManager;
use super::move_and_build_behaviour::MoveAndBuildBehaviour;
use super::move_behaviour::{MoveBehaviour, Result};
use super::work_behaviour::{IWorkBehaviour, WorkResult};
use crate::building::Building;
use crate::building::{Field, FieldState};
use crate::resources::inventory::{Inventory, InventoryResource};
use godot::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FarmerState {
    Idle,
    FieldBuilding,
    FieldRemoving,
    ReturningToHome,
}

pub struct FarmerBehaviourConfig {
    pub max_field_count: usize,
    pub field_building_radius: f32,
}

pub struct FarmerBehaviour {
    state: FarmerState,
    field_build_behaviour: MoveAndBuildBehaviour<Field>,
    move_behaviour: MoveBehaviour,
    fields: Vec<Gd<Field>>,
    config: FarmerBehaviourConfig,
    removing_field: Option<Gd<Field>>,
    inventory: Inventory,
    agent_name: String,
    home: Option<Gd<Building>>,
    parent_node: Option<Gd<Node>>,
}

impl IWorkBehaviour for FarmerBehaviour {
    fn work(&mut self, delta: f64, agent_position: Vector2) -> WorkResult {
        loop {
            godot_print!("Agent {}: Farmer state {}", self.agent_name, format!("{:?}", self.state));
            match self.state {
                FarmerState::Idle => {
                    if self.is_any_field_completed() {
                        self.start_field_removing(agent_position);
                    } else if self.fields.len() < self.config.max_field_count {
                        self.start_field_building(agent_position);
                    } else {
                        return WorkResult { result: Result::Success, next_position: None };
                    }
                }
                FarmerState::FieldBuilding => {
                    let (result, next_position) = self.field_build_behaviour.build(delta);
                    match result {
                        Result::Running => {
                            return WorkResult { result, next_position };
                        }
                        Result::Success => {
                            godot_print!("Agent {} Field build complete", self.agent_name);
                            self.state = FarmerState::Idle;
                            return WorkResult { result: Result::Success, next_position: next_position };
                        }
                    }
                }
                FarmerState::FieldRemoving => {
                    let (result, next_position) = self.field_build_behaviour.build(delta);
                    match result {
                        Result::Running => {
                            return WorkResult { result, next_position };
                        }
                        Result::Success => {
                            godot_print!("Agent {} Field removing complete", self.agent_name);
                            self.finish_field_removing();
                            self.start_returning_to_home(agent_position);
                        }
                    }
                }
                FarmerState::ReturningToHome => {
                    let (result, next_position) = self.move_behaviour.move_agent(delta);
                    match result {
                        Result::Running => {
                            return WorkResult { result, next_position: Some(next_position) };
                        }
                        Result::Success => {
                            self.finish_returning_to_home();
                            return WorkResult { result: Result::Success, next_position: Some(next_position) };
                        }
                    }
                }
            }
        }
    }

    fn start_work(&mut self, home: Gd<Building>, agent_name: String, parent_node: Option<Gd<Node>>) {
        self.home = Some(home);
        self.state = FarmerState::Idle;
        self.agent_name = agent_name;
        self.parent_node = parent_node;
    }

    fn is_work_available(&self) -> bool {
        self.fields.len() < self.config.max_field_count || self.is_any_field_completed()
    }
}

impl FarmerBehaviour {
    pub fn new(
        field_build_behaviour: MoveAndBuildBehaviour<Field>,
        move_behaviour: MoveBehaviour,
        farmer_config: FarmerBehaviourConfig,
    ) -> Self {
        Self {
            state: FarmerState::Idle,
            field_build_behaviour: field_build_behaviour,
            move_behaviour: move_behaviour,
            fields: Vec::new(),
            config: farmer_config,
            removing_field: None,
            inventory: Inventory::new(),
            agent_name: "".to_string(),
            home: None,
            parent_node: None,
        }
    }

    fn start_field_building(&mut self, agent_position: Vector2) {
        godot_print!("Agent {}: Starting field build", self.agent_name);
        let field = self.field_build_behaviour.start_construction(agent_position, self.agent_name.clone(), self.parent_node.clone());
        self.fields.push(field);
        self.state = FarmerState::FieldBuilding;
    }

    fn is_any_field_completed(&self) -> bool {
        self.fields.iter().any(|field| field.bind().state == FieldState::Grown)
    }

    fn start_field_removing(&mut self, agent_position: Vector2) {
        godot_print!("Agent {}: Starting field removing", self.agent_name);
        let field = self.fields.iter().find(|field| field.bind().state == FieldState::Grown).unwrap();
        self.removing_field = Some(field.clone());
        self.field_build_behaviour.start_deconstruction(field.clone(), agent_position);
        self.state = FarmerState::FieldRemoving;
    }

    fn finish_field_removing(&mut self) {
        godot_print!("Agent {} Field removing complete", self.agent_name);
        let mut free_space_manager = FreeSpaceManager::singleton();
        let field_instance_id = self.removing_field.as_ref().unwrap().instance_id();
        self.fields.retain(|field| field.instance_id() != field_instance_id);
        free_space_manager
            .bind_mut()
            .remove_occupied_position(self.removing_field.as_ref().unwrap().bind().base().get_position());
        self.removing_field.as_mut().unwrap().bind_mut().base_mut().queue_free();
        self.removing_field = None;
        self.inventory.add(InventoryResource::Wheat, 1);
    }

    fn start_returning_to_home(&mut self, agent_position: Vector2) {
        if self.home.is_none() {
            self.state = FarmerState::Idle;
            return;
        }
        godot_print!("Agent {} Starting returning to home", self.agent_name);
        self.move_behaviour.start_moving(agent_position, self.home.as_ref().unwrap().bind().base().get_position());
        self.state = FarmerState::ReturningToHome;
    }

    fn finish_returning_to_home(&mut self) {
        godot_print!("Agent {} Returning to home complete", self.agent_name);
        self.home.as_mut().unwrap().bind_mut().inventory.move_full_inventory_from(&mut self.inventory);
        self.state = FarmerState::Idle;
    }
}
