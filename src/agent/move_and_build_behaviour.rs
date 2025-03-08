use godot::prelude::*;

use super::move_behaviour::{MoveBehaviour, Result};
use crate::building::IBuilding;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    IdleState,
    MovingState,
    BuildingState,
}

pub struct MoveAndBuildBehaviour<T: IBuilding> {
    state: State,
    move_behaviour: MoveBehaviour,
    building: Option<Gd<T>>,
    build_offset: Vector2,
    agent_name: String,
    is_construction: bool,
    building_progress: f32,
    building_duration: f32,
}

impl<T: IBuilding> MoveAndBuildBehaviour<T> {
    pub fn new() -> Self {
        Self {
            state: State::IdleState,
            move_behaviour: MoveBehaviour::new(),
            building: None,
            build_offset: Vector2::new(0., 100.),
            agent_name: String::new(),
            is_construction: true,
            building_progress: 0.0,
            building_duration: 2.0,
        }
    }

    pub fn set_agent_name(&mut self, agent_name: String) {
        self.agent_name = agent_name;
    }

    pub fn start_construction(&mut self, building: Gd<T>, current_position: Vector2) {
        godot_print!("Agent {}: Starting move to build", self.agent_name);
        self.is_construction = true;
        self.building_progress = 0.0;
        self.start(building, current_position);
    }

    pub fn start_deconstruction(&mut self, building: Gd<T>, current_position: Vector2) {
        godot_print!("Agent {}: Starting move to deconstruct", self.agent_name);
        self.is_construction = false;
        self.building_progress = self.building_duration;
        self.start(building, current_position);
    }

    fn start(&mut self, building: Gd<T>, current_position: Vector2) {
        let build_position = building.bind().base().get_position();
        self.building = Some(building);
        let build_move_target = build_position + self.build_offset;
        self.move_behaviour
            .start_moving(current_position, build_move_target);
        self.state = State::MovingState;
    }

    pub fn build(&mut self, delta: f64) -> (Result, Option<Vector2>) {
        if self.state == State::IdleState {
            return (Result::Success, None);
        }
        if self.state == State::MovingState {
            let (result, next_position) = self.move_behaviour.move_agent(delta);
            if result == Result::Running {
                return (result, Some(next_position));
            }
            godot_print!("Agent {}: Starting build", self.agent_name);
            self.state = State::BuildingState;
        }
        if self.state == State::BuildingState {
            let result = self.process_building(delta);
            if result == Result::Running {
                return (result, None);
            }
            self.building = None;
            self.state = State::IdleState;
        }
        (Result::Success, None)
    }

    fn process_building(&mut self, delta: f64) -> Result {
        if !self.building.is_some() {
            return Result::Success;
        }

        let progress = if self.is_construction {
            self.building_progress += delta as f32;
            (self.building_progress / self.building_duration).min(1.0)
        } else {
            self.building_progress -= delta as f32;
            (self.building_progress / self.building_duration).max(0.0)
        };

        self.building.as_mut().unwrap().bind_mut().build(progress);

        if self.is_construction {
            if self.building_progress >= self.building_duration {
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
