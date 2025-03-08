use std::collections::HashSet;

use godot::prelude::*;
use rand::seq::SliceRandom;
use godot::classes::Engine;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct FreeSpaceManager {
    #[base]
    base: Base<Object>,
    cell_x_size: f32,
    cell_y_size: f32,
    distance_step: i32,
    occupied_positions: HashSet<(i32, i32)>,
}

#[godot_api]
impl IObject for FreeSpaceManager {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            cell_x_size: 200.0,
            cell_y_size: 200.0,
            distance_step: 3,
            occupied_positions: HashSet::new(),
        }
    }
}

#[godot_api]
impl FreeSpaceManager {
    pub fn singleton() -> Gd<FreeSpaceManager> {
        Engine::singleton()
            .get_singleton(&StringName::from("FreeSpaceManager"))
            .expect("FreeSpaceManager singleton not found").try_cast::<FreeSpaceManager>().unwrap()
    }

    pub fn add_occupied_position(&mut self, position: Vector2) {
        let (cell_x, cell_y) = self.cell_id_from_position(position);
        self.occupied_positions.insert((cell_x, cell_y));
    }

    pub fn remove_occupied_position(&mut self, position: Vector2) {
        let (cell_x, cell_y) = self.cell_id_from_position(position);
        self.occupied_positions.remove(&(cell_x, cell_y));
    }

    pub fn find_random_free_position_near(&self, target: Vector2, radius: f32) -> Vector2 {
        let (target_cell_x, target_cell_y) = self.cell_id_from_position(target);
        let reference_distance = (radius / self.cell_x_size).ceil() as i32;

        // Create a list of positions to check in expanding square pattern
        let mut available_cells = Vec::new();
        let mut distance_min = 0;
        let mut distance_max = reference_distance;
        while available_cells.len() == 0 {
            available_cells = self.expand_available_cells(target_cell_x, target_cell_y, distance_min, distance_max, reference_distance);
            distance_min = distance_max + 1;
            distance_max = distance_max + self.distance_step;
        }

        // Shuffle the positions
        let mut rng = rand::thread_rng();
        let cell = available_cells.choose_weighted(&mut rng, |p| p.2).unwrap();
        self.position_from_cell_id(cell.0, cell.1)
    }

    fn cell_id_from_position(&self, position: Vector2) -> (i32, i32) {
        let cell_x = ((position.x - self.cell_x_size / 2.) / self.cell_x_size).floor() as i32;
        let cell_y = ((position.y - self.cell_y_size / 2.) / self.cell_y_size).floor() as i32;
        (cell_x, cell_y)
    }

    fn position_from_cell_id(&self, cell_x: i32, cell_y: i32) -> Vector2 {
        Vector2::new(
            (cell_x as f32 * self.cell_x_size) + self.cell_x_size / 2.,
            (cell_y as f32 * self.cell_y_size) + self.cell_y_size / 2.
        )
    }

    fn expand_available_cells(&self, target_cell_x: i32, target_cell_y: i32, distance_min: i32, distance_max: i32, reference_distance: i32) -> Vec<(i32, i32, f32)> {
        let mut check_cells = Vec::new();

        for cell_x in distance_min..=distance_max {
            for cell_y in -distance_max..=distance_max {
                check_cells.push((cell_x, cell_y));
            }
        }

        for cell_x in -distance_max..=-distance_min {
            for cell_y in -distance_max..=distance_max {
                check_cells.push((cell_x, cell_y));
            }
        }

        for cell_y in distance_min..=distance_max {
            for cell_x in -distance_min + 1..=distance_min - 1 {
                check_cells.push((cell_x, cell_y));
            }
        }

        for cell_y in -distance_max..=-distance_min {
            for cell_x in -distance_min + 1..=distance_min - 1 {
                check_cells.push((cell_x, cell_y));
            }
        }

        check_cells
            .iter()
            .filter(|(cell_x, cell_y)| !self.occupied_positions.contains(&(target_cell_x + cell_x, target_cell_y + cell_y)))
            .map(|(cell_x, cell_y)| (target_cell_x + cell_x, target_cell_y + cell_y, ((-(cell_x.pow(2) + cell_y.pow(2)) as f32) / (reference_distance as f32).powf(2.0)).exp()))
            .collect()
    }
}

