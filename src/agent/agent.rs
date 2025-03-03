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
    building_house: Option<Gd<Sprite2D>>,
    building_offset: Vector2,
    building_progress: f32,
    building_duration: f32,
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
            building_house: None,
            building_offset: Vector2::new(50.0, -50.0),
            building_progress: 0.0,
            building_duration: 2.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == State::Idle {
            self.start_moving();
        }
        if self.state == State::Moving {
            let result = self.move_agent(delta);
            if result == Result::Success {
                godot_print!("Reached target");
                godot_print!("Starting build");
                self.start_building();
            } else {
                return;
            }
        }
        if self.state == State::Building {
            let result = self.build(delta);
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

    fn start_building(&mut self) {
        godot_print!("Start building");
        let mut sprite = Sprite2D::new_alloc();
        let path = "res://.godot/imported/farmer_tent.png-b0a81620f2308971a68ea826e6d01872.ctex";
        
        // Load the texture using ResourceLoader instead of setting path directly
        let texture = ResourceLoader::singleton()
            .load(path)
            .expect("Failed to load texture") // Handle the error appropriately in production code
            .cast::<CompressedTexture2D>();
        
        sprite.set_texture(&texture);
        sprite.set_scale(Vector2::new(0.25, 0.25));
        sprite.set_position(self.base().get_position() + self.building_offset);
        sprite.set_z_index(0);
        
        // Create and set up the shader
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
        self.base_mut().get_parent().expect("Parent not found").add_child(&sprite_node);
        
        self.building_house = Some(sprite);
        self.building_progress = 0.0;
        self.state = State::Building;
    }

    fn build(&mut self, delta: f64) -> Result {
        if !self.building_house.is_some() {
            return Result::Success;
        }
        self.building_progress += delta as f32; // Approximately one frame at 60 FPS
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

    fn start_moving(&mut self) {
        godot_print!("Start moving");
        self.moving_time = 0.0;
        self.move_reference_position = self.base().get_position();
        self.state = State::Moving;
    }
}