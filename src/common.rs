use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


pub type EntityId = usize;
pub type Vec2 = (f32, f32);

#[wasm_bindgen]
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum EntityType {
    Ship = 0,
    Enemy = 1,
    Laser = 2,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Entity {
    pub kind: EntityType,
    pub x: i32,
    pub y: i32,
    pub draw: bool
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


