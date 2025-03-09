use godot::prelude::*;

use crate::behaviour::move_and_build_behaviour::MoveAndBuildBehaviour;
use crate::building::Building;

use super::move_behaviour::Result;
use super::work_behaviour::IWorkBehaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum AgentState {
    Idle,
    HomeBuilding,
    Working,
}

pub struct AgentBehaviourResult {
    pub next_position: Option<Vector2>,
}

pub struct AgentBehaviour<T: IWorkBehaviour> {
    state: AgentState,
    home_build_behaviour: MoveAndBuildBehaviour<Building>,
    home: Option<Gd<Building>>,
    work_behaviour: T,
    agent_name: String,
    parent_node: Option<Gd<Node>>,
}

impl<T: IWorkBehaviour> AgentBehaviour<T> {
    pub fn new(home_build_behaviour: MoveAndBuildBehaviour<Building>, work_behaviour: T) -> Self {
        Self {
            state: AgentState::Idle,
            home_build_behaviour: home_build_behaviour,
            home: None,
            work_behaviour: work_behaviour,
            agent_name: "".to_string(),
            parent_node: None,
        }
    }

    pub fn tick(&mut self, delta: f64, agent_position: Vector2) -> AgentBehaviourResult {
        loop {
            match self.state {
                AgentState::Idle => {
                    if self.home.is_none() {
                        self.start_home_building(agent_position);
                    } else if self.work_behaviour.is_work_available() {
                        self.start_working();
                    }
                    return AgentBehaviourResult { next_position: None };
                }
                AgentState::HomeBuilding => {
                    let (result, next_position) = self.home_build_behaviour.build(delta);
                    match result {
                        Result::Running => {
                            return AgentBehaviourResult { next_position: next_position };
                        }
                        Result::Success => {
                            godot_print!("Agent {} Home build complete", self.agent_name);
                            self.state = AgentState::Idle;
                        }
                    }
                }
                AgentState::Working => {
                    let work_result = self.work_behaviour.work(delta, agent_position);
                    match work_result.result {
                        Result::Running => {
                            return AgentBehaviourResult { next_position: work_result.next_position };
                        }
                        Result::Success => {
                            self.state = AgentState::Idle;
                        }
                    }
                }
            }
        }
    }

    pub fn start(&mut self, agent_name: String, parent_node: Option<Gd<Node>>) {
        self.agent_name = agent_name;
        self.state = AgentState::Idle;
        self.parent_node = parent_node;
    }

    fn start_home_building(&mut self, agent_position: Vector2) {
        godot_print!("Agent {} Starting home build", self.agent_name);
        self.home = Some(self.home_build_behaviour.start_construction(
            agent_position,
            self.agent_name.clone(),
            self.parent_node.clone(),
        ));
        self.state = AgentState::HomeBuilding;
    }

    fn start_working(&mut self) {
        godot_print!("Agent {} Starting work", self.agent_name);
        self.work_behaviour.start_work(
            self.home.as_ref().unwrap().clone(),
            self.agent_name.clone(),
            self.parent_node.clone(),
        );
        self.state = AgentState::Working;
    }
}
