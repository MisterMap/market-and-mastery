use godot::prelude::*;
use godot::{builtin::Vector2, classes::{Sprite2D, ISprite2D}};

use super::{IBuilding, BuildingConfig};

#[derive(PartialEq, Eq, Clone, Copy)]
enum FieldState {
    Growing,
    Grown,
}

pub fn field_building_config() -> BuildingConfig {
    BuildingConfig {
        sprite_path: "res://.godot/imported/field.png-e3ee637cd0bc190899026182c03fbba0.ctex".into(),
        scale: Vector2::new(0.15, 0.15),
        building_name: "Field".into(),
    }
}

pub fn empty_field_building_config() -> BuildingConfig {
    BuildingConfig {
        sprite_path: "res://.godot/imported/empty_field.png-63272e1c00bbd5487b70086bc0094907.ctex".into(),
        scale: Vector2::new(0.15, 0.15),
        building_name: "Empty Field".into(),
    }
}


#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Field {
    #[base]
    base: Base<Sprite2D>,
    grow_progress: f32,
    grow_duration: f32,
    state: FieldState,
}

#[godot_api]
impl ISprite2D for Field {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            grow_progress: 0.0,
            grow_duration: 10.0,
            state: FieldState::Growing,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.state == FieldState::Growing {
            self.grow_progress += delta as f32/ self.grow_duration;
            if self.grow_progress >= 1.0 {
                self.grow();
            }
        }   
    }
}

impl IBuilding for Field {}

impl Field {
    pub fn from_position(position: Vector2) -> Gd<Self> {
        IBuilding::from_config_and_position(empty_field_building_config(), position)
    }

    pub fn grow(&mut self) {
        godot_print!("Field growing");
        self.set_new_config(field_building_config());
        self.state = FieldState::Grown;
    }
}
