use godot::prelude::*;
use godot::classes::{CompressedTexture2D, ResourceLoader, Sprite2D, Node, Shader, ShaderMaterial};

use super::move_behaviour::Result;

pub struct BuildBehaviour {
    building_house: Option<Gd<Sprite2D>>,
    building_offset: Vector2,
    building_progress: f32,
    building_duration: f32,
}

impl BuildBehaviour {
    pub fn new() -> Self {
        Self {
            building_house: None,
            building_offset: Vector2::new(50.0, -50.0),
            building_progress: 0.0,
            building_duration: 2.0,
        }
    }

    pub fn start_building(&mut self, mut parent: Gd<Node>, position: Vector2) {
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

    pub fn build(&mut self, delta: f64) -> Result {
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