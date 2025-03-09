use godot::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Result {
    Success,
    Running,
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub struct MoveBehaviourConfig {
    pub speed: f32,
    pub max_step_height: f32,
    pub step_period: f32,
}

#[derive(Clone)]
pub struct MoveBehaviour {
    moving_time: f32,
    move_reference_position: Option<Vector2>,
    target: Option<Vector2>,
    config: MoveBehaviourConfig,
}

impl MoveBehaviour {
    pub fn new(config: MoveBehaviourConfig) -> Self {
        Self { move_reference_position: None, target: None, moving_time: 0.0, config }
    }

    pub fn move_agent(&mut self, delta: f64) -> (Result, Vector2) {
        let rotation = self.move_reference_position.unwrap().angle_to_point(self.target.unwrap());
        let delta_distance = self.config.speed * delta as f32;
        let distance_to_target = self.move_reference_position.unwrap().distance_to(self.target.unwrap());

        if distance_to_target < delta_distance {
            return (Result::Success, self.target.unwrap());
        };

        self.move_reference_position =
            Some(self.move_reference_position.unwrap() + Vector2::RIGHT.rotated(rotation) * delta_distance);
        let next_position = self.move_reference_position.unwrap() + Vector2::UP * self.step_height();
        self.moving_time += delta as f32;
        (Result::Running, next_position)
    }

    fn step_height(&self) -> f32 {
        self.config.max_step_height * (self.moving_time / self.config.step_period).sin().abs()
    }

    pub fn start_moving(&mut self, current_position: Vector2, target: Vector2) {
        self.moving_time = 0.0;
        self.move_reference_position = Some(current_position);
        self.target = Some(target);
    }
}
