use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    Moving,
    Building,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Result {
    Success,
    Running,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Agent {
    speed: f32,
    state: State,
    base: Base<Sprite2D>,
    target: Vector2,
    moving_time: f32,
    max_step_hieght: f32,
    step_period: f32,
    move_reference_position: Vector2,
}


#[godot_api]
impl ISprite2D for Agent {
    fn init(base: Base<Sprite2D>) -> Self { 
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            speed: 100.0,
            state: State::Idle,
            base,
            target: Vector2::new(1000.0, 500.0),
            moving_time: 0.0,
            max_step_hieght: 20.0,
            step_period: 0.1,
            move_reference_position: Vector2::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.start_moving();
        }
        if self.state == State::Moving {
            godot_print!("Moving from: {}", self.base().get_position());
            let result = self.move_agent(delta);
            if result == Result::Success {
                godot_print!("Reached target");
                godot_print!("Starting build");
                self.state = State::Building;
            } else {
                return;
            }
        }
        if self.state == State::Building {
            let result = self.build();
            if result == Result::Success {
                godot_print!("Build complete");
                self.state = State::Moving;
            } else {
                return;
            }
        }
    }
}

#[godot_api]
impl Agent {
    fn move_agent(&mut self, delta: f64) -> Result {
        let rotation = self.move_reference_position.angle_to_point(self.target);
        let delta_distance = self.speed * delta as f32;
        let distance_to_target = self.move_reference_position.distance_to(self.target);
        if distance_to_target < delta_distance {
            let target = self.target;
            self.base_mut().set_position(target);
            return Result::Success;
        };
        self.move_reference_position = self.move_reference_position + Vector2::RIGHT.rotated(rotation) * delta_distance;
        let next_position = self.move_reference_position + Vector2::UP * self.step_height();
        self.base_mut().set_position(next_position);
        self.moving_time += delta as f32;
        Result::Running
    }

    fn step_height(&self) -> f32 {
        self.max_step_hieght * (self.moving_time / self.step_period).sin().abs()
    }

    fn build(&mut self) -> Result {
        godot_print!("Building mode activated");
        Result::Success
    }

    fn start_moving(&mut self) {
        godot_print!("Start moving");
        self.moving_time = 0.0;
        self.move_reference_position = self.base().get_position();
        self.state = State::Moving;
    }
}