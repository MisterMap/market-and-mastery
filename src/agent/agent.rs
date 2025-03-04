use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

use super::move_behaviour::{MoveBehaviour, Result};
use super::build_behaviour::BuildBehaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    Moving,
    Building,
    Finished,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Agent {
    state: State,
    base: Base<Sprite2D>,
    move_behaviour: MoveBehaviour,
    build_behaviour: BuildBehaviour,
    target: Vector2,
}

#[godot_api]
impl ISprite2D for Agent {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");
        Self {
            state: State::Idle,
            base,
            move_behaviour: MoveBehaviour::new(),
            build_behaviour: BuildBehaviour::new(),
            target: Vector2::new(700., 300.)
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.start_moving();
        }
        if self.state == State::Moving {
            let (result, next_position) = self.move_behaviour.move_agent(delta);
            self.base_mut().set_position(next_position);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Reached target");
                self.start_building();
            }
        }
        if self.state == State::Building {
            let result = self.build_behaviour.build(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Build complete");
                self.state = State::Finished;
            }
        }
    }
}

impl Agent {
    fn start_moving(&mut self) {
        self.move_behaviour.start_moving(self.base().get_position(), self.target);
        self.state = State::Moving;
    }

    fn start_building(&mut self) {
        godot_print!("Starting build");
        self.build_behaviour.start_building(self.base().get_parent().expect("Parent not found"), self.base().get_position());
        self.state = State::Building;
    }
}