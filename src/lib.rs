mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(i32)]
#[derive(Clone)]
pub enum EntityType {
    Ship = 0,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Entity {
    kind: EntityType,
    x: i32,
    y: i32,
    draw: bool
}

#[wasm_bindgen]
impl Entity {
    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y
    }
}

#[wasm_bindgen]
pub struct Universe {
    width:  u32,
    height: u32,
    entities: Vec<Entity>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut u = Universe {
            width:  20,
            height: 20,
            entities: vec![]
        };

        u.new_ship();
        return u;
    }

    fn new_ship(&mut self) {
        self.entities.push(Entity { kind: EntityType::Ship, x: 0, y: 0, draw: true })
    }

    fn get_ship_mut(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(0)
    }

    pub fn tick(&mut self) {
        //let width = self.width;
        //if let Some(ship) = self.get_ship_mut() {
            //let x = ship.get_x();
            //ship.set_x(x + 1 % width)
        //}
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn entities(&self) -> *const Entity {
        self.entities.as_ptr()
    }

    pub fn n_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn left_arrow_key(&mut self) {
        let width = self.width;
        if let Some(ship) = self.get_ship_mut() {
            let x = ship.get_x();
            ship.set_x((x - 1).rem_euclid(width as i32));
        }
    }
    pub fn right_arrow_key(&mut self) {
        let width = self.width;
        if let Some(ship) = self.get_ship_mut() {
            let x = ship.get_x();
            ship.set_x((x + 1) % width as i32)
        }
    }
    pub fn up_arrow_key(&mut self) {
        let height = self.height;
        if let Some(ship) = self.get_ship_mut() {
            let y = ship.get_y();
            ship.set_y((y + 1) % height as i32)
        }
    }
    pub fn down_arrow_key(&mut self) {
        let height = self.height;
        if let Some(ship) = self.get_ship_mut() {
            let y = ship.get_y();
            ship.set_y((y - 1).rem_euclid(height as i32));
        }
    }
}
