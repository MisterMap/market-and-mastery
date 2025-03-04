use godot::classes::CompressedTexture2D;
use godot::classes::ResourceLoader;
use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;
use godot::classes::Node;
use godot::classes::Shader;
use godot::classes::ShaderMaterial;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Idle,
    Moving,
    Building,
    Finished,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Result {
    Success,
    Running,
}

struct MoveBehaviour {
    speed: f32,
    moving_time: f32,
    max_step_height: f32,
    step_period: f32,
    move_reference_position: Option<Vector2>,
    target: Option<Vector2>,
}

impl MoveBehaviour {
    fn new() -> Self {
        Self {
            speed: 100.0,
            moving_time: 0.0,
            max_step_height: 20.0,
            step_period: 0.1,
            move_reference_position: None,
            target: None,
        }
    }

    fn move_agent(&mut self, delta: f64) -> (Result, Vector2) {
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

    fn start_moving(&mut self, current_position: Vector2, target: Vector2) {
        self.moving_time = 0.0;
        self.move_reference_position = Some(current_position);
        self.target = Some(target);
    }
}

struct BuildBehaviour {
    building_house: Option<Gd<Sprite2D>>,
    building_offset: Vector2,
    building_progress: f32,
    building_duration: f32,
}

impl BuildBehaviour {
    fn new() -> Self {
        Self {
            building_house: None,
            building_offset: Vector2::new(50.0, -50.0),
            building_progress: 0.0,
            building_duration: 2.0,
        }
    }

    fn start_building(&mut self, mut parent: Gd<Node>, position: Vector2) {
        let mut sprite = Sprite2D::new_alloc();
        let path = "res://.godot/imported/farmer_tent.png-b0a81620f2308971a68ea826e6d01872.ctex";
        
        let texture = ResourceLoader::singleton()
            .load(path)
            .expect("Failed to load texture")
            .cast::<CompressedTexture2D>();
        
        sprite.set_texture(&texture);
        sprite.set_scale(Vector2::new(0.25, 0.25));
        sprite.set_position(position + self.building_offset);
        sprite.set_z_index(0);
        
        let shader_code = "
            shader_type canvas_item;
            uniform float progress : hint_range(0.0, 1.0) = 0.0;
            
            void fragment() {
                vec4 texture_color = texture(TEXTURE, UV);
                float mask = step(1.0 - UV.y, progress);
                COLOR = texture_color * mask;
            }
        ";
        
        let mut shader = Shader::new_gd();
        shader.set_code(shader_code);
        
        let mut shader_material = ShaderMaterial::new_gd();
        shader_material.set_shader(&shader);
        
        sprite.set_material(&shader_material);
        
        let sprite_node = sprite.clone().upcast::<Node>();
        parent.add_child(&sprite_node);
        
        self.building_house = Some(sprite);
        self.building_progress = 0.0;
    }

    fn build(&mut self, delta: f64) -> Result {
        if !self.building_house.is_some() {
            return Result::Success;
        }
        self.building_progress += delta as f32;
        let progress = (self.building_progress / self.building_duration).min(1.0);
            
        if let Some(material) = self.building_house.as_ref().unwrap().get_material() {
            let mut shader_material = material.cast::<ShaderMaterial>();
            shader_material.set_shader_parameter("progress", &Variant::from(progress));
        }
        
        if self.building_progress >= self.building_duration {
            return Result::Success;
        }
        
        Result::Running
    }
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