use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::behaviour::agent_behaviour::AgentBehaviour;
use crate::behaviour::behaviour_regestry::make_farmer_agent_behaviour;
use crate::behaviour::farmer_behaviour::FarmerBehaviour;

#[derive(PartialEq, Eq, Clone, Copy)]
enum FarmerState {
    Starting,
    Acting,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Farmer {
    base: Base<Sprite2D>,
    behaviour: AgentBehaviour<FarmerBehaviour>,
    state: FarmerState,
}

#[godot_api]
impl ISprite2D for Farmer {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");
        let agent_name = base.to_gd().get_name().to_string();
        godot_print!("Agent name: {}", agent_name);
        Self { base, behaviour: make_farmer_agent_behaviour(), state: FarmerState::Starting }
    }

    fn physics_process(&mut self, delta: f64) {
        match self.state {
            FarmerState::Starting => {
                self.behaviour.start(self.base().get_name().to_string(), self.base().get_parent());
                self.state = FarmerState::Acting;
            }
            FarmerState::Acting => {
                let result = self.behaviour.tick(delta, self.base().get_position());
                match result.next_position {
                    Some(next_position) => {
                        self.base_mut().set_position(next_position);
                    }
                    None => {}
                }
            }
        }
    }
}
