use godot::prelude::*;

use super::{
    free_space_manager::FreeSpaceManager,
    move_behaviour::{MoveBehaviour, Result},
};
use crate::building::IBuilding;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    IdleState,
    MovingState,
    BuildingState,
}

#[derive(Clone)]
pub struct MoveAndBuildBehaviourConfig {
    pub building_radius: f32,
    pub build_offset: Vector2,
    pub building_duration: f32,
}

pub struct MoveAndBuildBehaviour<T: IBuilding> {
    state: State,
    move_behaviour: MoveBehaviour,
    building: Option<Gd<T>>,
    agent_name: String,
    is_construction: bool,
    building_progress: f32,
    config: MoveAndBuildBehaviourConfig,
}

impl<T: IBuilding + Inherits<Node>> MoveAndBuildBehaviour<T> {
    pub fn new(move_behaviour: MoveBehaviour, config: MoveAndBuildBehaviourConfig) -> Self {
        Self {
            state: State::IdleState,
            move_behaviour,
            building: None,
            agent_name: String::new(),
            is_construction: true,
            building_progress: 0.0,
            config,
        }
    }

    fn calculate_free_space_position(&self, target_position: Vector2, radius: f32) -> Vector2 {
        let mut free_space_manager = FreeSpaceManager::singleton();
        let build_position = free_space_manager.bind().find_random_free_position_near(target_position, radius);
        free_space_manager.bind_mut().add_occupied_position(build_position);
        build_position
    }

    fn add_building_to_parent(&mut self, building: &Gd<T>, mut parent_node: Option<Gd<Node>>) {
        let node = building.clone().upcast::<Node>();
        parent_node.as_mut().unwrap().add_child(&node);
    }

    pub fn set_agent_name(&mut self, agent_name: String) {
        self.agent_name = agent_name;
    }

    pub fn start_construction(&mut self, current_position: Vector2, agent_name: String, parent_node: Option<Gd<Node>>) -> Gd<T> {
        self.agent_name = agent_name;
        let building_position = self.calculate_free_space_position(current_position, self.config.building_radius);
        let building = T::from_position(building_position);
        self.add_building_to_parent(&building, parent_node);
        self.is_construction = true;
        self.building_progress = 0.0;
        self.start_move_to_build(building.clone(), current_position);
        building
    }

    pub fn start_deconstruction(&mut self, building: Gd<T>, current_position: Vector2) {
        godot_print!("Agent {}: Starting move to deconstruct", self.agent_name);
        self.is_construction = false;
        self.building_progress = self.config.building_duration;
        self.start_move_to_build(building, current_position);
    }

    fn start_move_to_build(&mut self, building: Gd<T>, current_position: Vector2) {
        godot_print!("Agent {}: Starting move to build", self.agent_name);
        let build_position = building.bind().base().get_position();
        self.building = Some(building);
        let build_move_target = build_position + self.config.build_offset;
        self.move_behaviour.start_moving(current_position, build_move_target);
        self.state = State::MovingState;
    }

    pub fn build(&mut self, delta: f64) -> (Result, Option<Vector2>) {
        loop {
            godot_print!("Agent {}: Building state {}", self.agent_name, format!("{:?}", self.state));
            match self.state {
                State::IdleState => {
                    return (Result::Success, None);
                }
                State::MovingState => {
                    let (result, next_position) = self.move_behaviour.move_agent(delta);
                    godot_print!("Agent {}: Moving result {}", self.agent_name, format!("{:?}", result));
                    if result == Result::Running {
                        return (result, Some(next_position));
                    }
                    godot_print!("Agent {}: Starting build", self.agent_name);
                    self.state = State::BuildingState;
                }
                State::BuildingState => {
                    let result = self.process_building(delta);
                    if result == Result::Running {
                        return (result, None);
                    }
                    self.building = None;
                    self.state = State::IdleState;
                }
            }
        }
    }

    fn process_building(&mut self, delta: f64) -> Result {
        if !self.building.is_some() {
            return Result::Success;
        }

        let progress = if self.is_construction {
            self.building_progress += delta as f32;
            (self.building_progress / self.config.building_duration).min(1.0)
        } else {
            self.building_progress -= delta as f32;
            (self.building_progress / self.config.building_duration).max(0.0)
        };

        self.building.as_mut().unwrap().bind_mut().build(progress);

        if self.is_construction {
            if self.building_progress >= self.config.building_duration {
                self.building.as_mut().unwrap().bind_mut().set_completed();
                return Result::Success;
            }
        } else {
            if self.building_progress <= 0.0 {
                return Result::Success;
            }
        }

        Result::Running
    }
}
