use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

use super::free_space_manager::FreeSpaceManager;
use super::move_behaviour::{MoveBehaviour, Result};
use super::build_behaviour::BuildBehaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    BuildHomeMoving,
    HomeBuilding,
    Finished,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Agent {
    state: State,
    base: Base<Sprite2D>,
    move_behaviour: MoveBehaviour,
    build_behaviour: BuildBehaviour,
    build_home_move_target: Vector2,
    build_home_position: Vector2,
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
            build_home_move_target: Vector2::ZERO,
            build_home_position: Vector2::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.start_build_home_moving();
        }
        if self.state == State::BuildHomeMoving {
            let (result, next_position) = self.move_behaviour.move_agent(delta);
            self.base_mut().set_position(next_position);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Reached target", self.base().get_name());
                self.start_home_building();
            }
        }
        if self.state == State::HomeBuilding {
            let result = self.build_behaviour.build(delta);
            if result == Result::Running {
                return;
            }
            if result == Result::Success {
                godot_print!("Agent {} Build complete", self.base().get_name());
                self.state = State::Finished;
            }
        }
    }
}

impl Agent {
    fn start_build_home_moving(&mut self) {
        godot_print!("Agent {} Starting build move", self.base().get_name());
        let mut free_space_manager = FreeSpaceManager::singleton();
        self.build_home_position = free_space_manager.bind().find_random_free_position_near(self.base().get_position());
        free_space_manager.bind_mut().add_occupied_position(self.build_home_position);
        self.build_home_move_target = self.build_home_position + Vector2::new(0., 100.);
        self.move_behaviour.start_moving(self.base().get_position(), self.build_home_move_target);
        self.state = State::BuildHomeMoving;
    }

    fn start_home_building(&mut self) {
        godot_print!("Agent {} Starting build", self.base().get_name());
        let parent = self.base().get_parent().expect("Parent not found");
        self.build_behaviour.start_building(parent, self.build_home_position);
        self.state = State::HomeBuilding;
    }
}