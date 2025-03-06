use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{CompressedTexture2D, ResourceLoader, Sprite2D, ISprite2D, Shader, ShaderMaterial};
use crate::building::BuildingConfig;
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Building {
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Building {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
        }
    }
}

pub trait IBuilding: ISprite2D + WithBaseField {
    fn from_config_and_position(config: BuildingConfig, position: Vector2) -> Gd<Self> {
        let mut building = Gd::from_init_fn(|base| {
            Self::init(base)
        });
        let path = config.sprite_path;
        
        let texture = ResourceLoader::singleton()
            .load(&path)
            .expect("Failed to load texture")
            .cast::<CompressedTexture2D>();
        
        building.bind_mut().base_mut().set_texture(&texture);
        building.bind_mut().base_mut().set_scale(config.scale);
        building.bind_mut().base_mut().set_position(position);
        building.bind_mut().base_mut().set_z_index(0);
        
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
        
        building.bind_mut().base_mut().set_material(&shader_material);
        
        building
    }

    fn set_new_config(&mut self, config: BuildingConfig) {
        let path = config.sprite_path;
        
        let texture = ResourceLoader::singleton()
            .load(&path)
            .expect("Failed to load texture")
            .cast::<CompressedTexture2D>();
        self.base_mut().set_texture(&texture);
        self.base_mut().set_scale(config.scale);
    }

    fn build(&mut self, progress: f32) {
        if let Some(material) = self.base().get_material() {
            let mut shader_material = material.cast::<ShaderMaterial>();
            shader_material.set_shader_parameter("progress", &Variant::from(progress));
        }
    }
}

impl IBuilding for Building {}

