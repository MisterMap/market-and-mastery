use crate::building::BuildingConfig;
use crate::resources::inventory::Inventory;
use godot::classes::{CompressedTexture2D, ISprite2D, Label, ResourceLoader, Shader, ShaderMaterial, Sprite2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BuildingState {
    Building,
    Completed,
}

impl IBuilding for Building {
    fn set_completed(&mut self) {
        self.state = BuildingState::Completed;
    }
}
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Building {
    #[base]
    base: Base<Sprite2D>,
    pub inventory: Inventory,
    resouce_label: Option<Gd<Label>>,
    resource_label_position: Vector2,
    state: BuildingState,
}

#[godot_api]
impl ISprite2D for Building {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            inventory: Inventory::new(),
            resouce_label: None,
            resource_label_position: Vector2::new(0.0, -600.0),
            state: BuildingState::Building,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.state == BuildingState::Building {
            return;
        }
        if self.resouce_label.is_none() {
            self.resouce_label = Some(Label::new_alloc());
            let node = self.resouce_label.as_mut().unwrap().clone().upcast::<Node>();
            self.base_mut().add_child(&node);
            self.resouce_label.as_mut().unwrap().set_position(self.resource_label_position);
            self.resouce_label.as_mut().unwrap().set_scale(Vector2::new(6.0, 6.0));
        }
        self.resouce_label.as_mut().unwrap().set_text(&self.inventory.to_string());
    }
}

pub trait IBuilding: ISprite2D + WithBaseField {
    fn from_config_and_position(config: BuildingConfig, position: Vector2) -> Gd<Self> {
        let mut building = Gd::from_init_fn(|base| Self::init(base));
        let path = config.sprite_path;

        let texture =
            ResourceLoader::singleton().load(&path).expect("Failed to load texture").cast::<CompressedTexture2D>();

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

        let texture =
            ResourceLoader::singleton().load(&path).expect("Failed to load texture").cast::<CompressedTexture2D>();
        self.base_mut().set_texture(&texture);
        self.base_mut().set_scale(config.scale);
    }

    fn build(&mut self, progress: f32) {
        if let Some(material) = self.base().get_material() {
            let mut shader_material = material.cast::<ShaderMaterial>();
            shader_material.set_shader_parameter("progress", &Variant::from(progress));
        }
    }

    fn set_completed(&mut self);
}
