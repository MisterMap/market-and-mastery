use godot::prelude::*;
use super::move_behaviour::Result;
use crate::building::Building;

pub struct BuildBehaviour {
    building: Option<Gd<Building>>,
    building_progress: f32,
    building_duration: f32,
}

impl BuildBehaviour {
    pub fn new() -> Self {
        Self {
            building: None,
            building_progress: 0.0,
            building_duration: 2.0,
        }
    }

    pub fn start_building(&mut self, building: Gd<Building>) {
        self.building = Some(building);
        self.building_progress = 0.0;
    }

    pub fn build(&mut self, delta: f64) -> Result {
        if !self.building.is_some() {
            return Result::Success;
        }
        self.building_progress += delta as f32;
        let progress = (self.building_progress / self.building_duration).min(1.0);
            
        self.building.as_mut().unwrap().bind_mut().process(progress);
        
        if self.building_progress >= self.building_duration {
            return Result::Success;
        }
        
        Result::Running
    }
} 