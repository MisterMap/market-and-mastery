use godot::builtin::Vector2;

use crate::building::{Building, Field};

use super::{
    agent_behaviour::AgentBehaviour,
    farmer_behaviour::{FarmerBehaviour, FarmerBehaviourConfig},
    move_and_build_behaviour::{MoveAndBuildBehaviour, MoveAndBuildBehaviourConfig},
    move_behaviour::{MoveBehaviour, MoveBehaviourConfig},
};

fn make_move_behaviour_config() -> MoveBehaviourConfig {
    MoveBehaviourConfig { speed: 100.0, max_step_height: 20.0, step_period: 0.1 }
}

fn make_farmer_behaviour_config() -> FarmerBehaviourConfig {
    FarmerBehaviourConfig { max_field_count: 3, field_building_radius: 100.0 }
}

fn make_field_build_behaviour_config() -> MoveAndBuildBehaviourConfig {
    MoveAndBuildBehaviourConfig {
        building_radius: 100.0,
        build_offset: Vector2::new(0.0, 100.0),
        building_duration: 1.0,
    }
}

fn make_home_build_behaviour_config() -> MoveAndBuildBehaviourConfig {
    MoveAndBuildBehaviourConfig {
        building_radius: 600.0,
        build_offset: Vector2::new(0.0, 100.0),
        building_duration: 2.0,
    }
}

fn make_move_behaviour() -> MoveBehaviour {
    MoveBehaviour::new(make_move_behaviour_config())
}

fn make_field_build_behaviour() -> MoveAndBuildBehaviour<Field> {
    MoveAndBuildBehaviour::new(make_move_behaviour(), make_field_build_behaviour_config())
}

fn make_home_build_behaviour() -> MoveAndBuildBehaviour<Building> {
    MoveAndBuildBehaviour::new(make_move_behaviour(), make_home_build_behaviour_config())
}

fn make_farmer_behaviour() -> FarmerBehaviour {
    FarmerBehaviour::new(make_field_build_behaviour(), make_move_behaviour(), make_farmer_behaviour_config())
}

pub fn make_farmer_agent_behaviour() -> AgentBehaviour<FarmerBehaviour> {
    AgentBehaviour::new(make_home_build_behaviour(), make_farmer_behaviour())
}
