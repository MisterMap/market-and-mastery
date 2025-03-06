use godot::prelude::*;

use super::move_behaviour::{MoveBehaviour, Result};
use super::build_behaviour::BuildBehaviour;
use crate::building::IBuilding;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    Moving,
    BuildingState,
}

pub struct MoveAndBuildBehaviour<T: IBuilding> {
    state: State,
    move_behaviour: MoveBehaviour,
    build_behaviour: BuildBehaviour<T>,
    building: Option<Gd<T>>,
    build_offset: Vector2,
    agent_name: String,
}

impl<T: IBuilding> MoveAndBuildBehaviour<T> {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            move_behaviour: MoveBehaviour::new(),
            build_behaviour: BuildBehaviour::new(),
            building: None,
            build_offset: Vector2::new(0., 100.),
            agent_name: String::new(),
        }
    }

    pub fn set_agent_name(&mut self, agent_name: String) {
        self.agent_name = agent_name;
    }

    pub fn start(&mut self, building: Gd<T>, current_position: Vector2) {
        godot_print!("Agent {}: Starting move to build", self.agent_name);
        let build_position = building.bind().base().get_position();
        self.building = Some(building);
        let build_move_target = build_position + self.build_offset;
        self.move_behaviour.start_moving(current_position, build_move_target);
        self.state = State::Moving;
    }

    fn start_building(&mut self) {
        godot_print!("Agent {}: Starting build", self.agent_name);
        self.build_behaviour.start_building(self.building.as_ref().unwrap().clone());
        self.state = State::BuildingState;
    }

    pub fn build(&mut self, delta: f64) -> (Result, Option<Vector2>) {
        if self.state == State::Idle {
            return (Result::Success, None);
        }
        if self.state == State::Moving {
            let (result, next_position) = self.move_behaviour.move_agent(delta);
            if result == Result::Running {
                return (result, Some(next_position));
            }
            self.start_building();
        }
        if self.state == State::BuildingState {
            let result = self.build_behaviour.build(delta);
            if result == Result::Running {
                return (result, None);
            }
            self.state = State::Idle;
        }
        (Result::Success, None) 
    }
} 