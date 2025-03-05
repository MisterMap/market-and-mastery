use godot::prelude::{Vector2, GString};

pub struct BuildingConfig {
    pub sprite_path: GString,
    pub scale: Vector2,
    pub building_name: GString,
}

pub fn home_building_config() -> BuildingConfig {
    BuildingConfig {
        sprite_path: "res://.godot/imported/farmer_tent.png-b0a81620f2308971a68ea826e6d01872.ctex".into(),
        scale: Vector2::new(0.25, 0.25),
        building_name: "Home".into(),
    }
}

pub fn field_building_config() -> BuildingConfig {
    BuildingConfig {
        sprite_path: "res://.godot/imported/field.png-e3ee637cd0bc190899026182c03fbba0.ctex".into(),
        scale: Vector2::new(0.15, 0.15),
        building_name: "Field".into(),
    }
}
