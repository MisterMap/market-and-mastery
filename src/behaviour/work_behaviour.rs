use godot::prelude::*;
use super::move_behaviour::Result;
use crate::building::Building;

pub struct WorkResult {
    pub result: Result,
    pub next_position: Option<Vector2>,
}

pub trait IWorkBehaviour {
    fn work(&mut self, delta: f64, agent_position: Vector2) -> WorkResult;
    fn start_work(&mut self, home: Gd<Building>, agent_name: String, parent_node: Option<Gd<Node>>);
    fn is_work_available(&self) -> bool;
}
