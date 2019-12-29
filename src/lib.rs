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
pub fn greet() {
    alert("Hello, galaga!");
}

#[wasm_bindgen]
#[repr(u32)]
#[derive(Clone)]
pub enum EntityType {
    Ship = 0,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Entity {
    kind: EntityType,
    x: u32,
    y: u32,
    draw: bool
}

#[wasm_bindgen]
impl Entity {
    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x % 20
    }
}

#[wasm_bindgen]
pub struct Universe {
    height: u32,
    width: u32,
    entities: Vec<Entity>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut u = Universe {
            height: 20,
            width: 20,
            entities: vec![]
        };

        u.new_ship();
        return u;
    }

    fn new_ship(&mut self) {
        self.entities.push(Entity { kind: EntityType::Ship, x: 0, y: 0, draw: true })
    }

    pub fn tick(&mut self) {
        if let Some(ship) = self.entities.get_mut(0) {
            let x = ship.get_x();
            ship.set_x(x + 1 % self.height)
        }
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
}
