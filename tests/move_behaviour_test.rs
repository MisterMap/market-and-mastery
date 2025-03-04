use market_and_mastery::agent::move_behaviour::{MoveBehaviour, Result};
use godot::prelude::*;
use approx::assert_relative_eq;


#[test]
fn test_move_agent_success() {
    let mut behaviour = MoveBehaviour::new();
    let current_pos = Vector2::new(0.0, 0.0);
    let target_pos = Vector2::new(10.0, 0.0);
    
    behaviour.start_moving(current_pos, target_pos);
    let (result, position) = behaviour.move_agent(0.2); // With speed 100, should reach target

    assert_eq!(result, Result::Success);
    assert_eq!(position, target_pos);
}

#[test]
fn test_move_agent_running() {
    let mut behaviour = MoveBehaviour::new();
    let current_pos = Vector2::new(0.0, 0.0);
    let target_pos = Vector2::new(100.0, 0.0);
    
    behaviour.start_moving(current_pos, target_pos);
    let (result, position) = behaviour.move_agent(0.1); // Should move 10 units

    assert_eq!(result, Result::Running);
    assert_relative_eq!(position.x, 10.0, epsilon = 0.001);
    // Y position will vary due to step height
}

