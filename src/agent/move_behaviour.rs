use godot::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Result {
    Success,
    Running,
}

pub struct MoveBehaviour {
    speed: f32,
    moving_time: f32,
    max_step_height: f32,
    step_period: f32,
    move_reference_position: Option<Vector2>,
    target: Option<Vector2>,
}

impl MoveBehaviour {
    pub fn new() -> Self {
        Self {
            speed: 100.0,
            moving_time: 0.0,
            max_step_height: 20.0,
            step_period: 0.1,
            move_reference_position: None,
            target: None,
        }
    }

    pub fn move_agent(&mut self, delta: f64) -> (Result, Vector2) {
        let rotation = self.move_reference_position.unwrap().angle_to_point(self.target.unwrap());
        let delta_distance = self.speed * delta as f32;
        let distance_to_target = self.move_reference_position.unwrap().distance_to(self.target.unwrap());
        
        if distance_to_target < delta_distance {
            return (Result::Success, self.target.unwrap());
        };
        
        self.move_reference_position = Some(self.move_reference_position.unwrap() + Vector2::RIGHT.rotated(rotation) * delta_distance);
        let next_position = self.move_reference_position.unwrap() + Vector2::UP * self.step_height();
        self.moving_time += delta as f32;
        (Result::Running, next_position)
    }

    fn step_height(&self) -> f32 {
        self.max_step_height * (self.moving_time / self.step_period).sin().abs()
    }

    pub fn start_moving(&mut self, current_position: Vector2, target: Vector2) {
        self.moving_time = 0.0;
        self.move_reference_position = Some(current_position);
        self.target = Some(target);
    }
} 