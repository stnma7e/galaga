extern crate web_sys;

mod utils;
mod common;
mod physics;
mod msg;

use wasm_bindgen::prelude::*;
use std::cmp::{ max, min };

use physics::{ PhysicsManager };
use common::{ Entity, EntityType, EntityId };
use msg::{ Msg };

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width:  u32,
    height: u32,
    max_ship: u32,
    entities: Vec<Entity>,
    phys_manager: PhysicsManager,
    current_msgs: Vec<Msg>,
    next_msgs: Vec<Msg>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let mut u = Universe {
            width:  1000,
            height: 1000,
            max_ship: 5,
            entities: vec![],
            phys_manager: PhysicsManager::new(),
            current_msgs: vec![],
            next_msgs: vec![],
        };

        u.entities.push(Entity { kind: EntityType::Ship, x: 0, y: 0, draw: true });
        u.entities.push(Entity { kind: EntityType::Enemy, x: 250, y: 400, draw: true });
        return u;
    }

    pub fn tick(&mut self, dt: u32) {
        let msgs = self.phys_manager.tick(dt, &mut self.entities, &self.current_msgs, self.width, self.height);
        self.next_msgs.extend(msgs.into_iter());

        for msg in &self.current_msgs {
            match msg {
                Msg::DeleteEntity(entity) => {
                    self.entities[*entity].draw = false;
                }
            }
        }

        self.current_msgs.clear();
        std::mem::swap(&mut self.current_msgs, &mut self.next_msgs);
        //log!("{}", self.current_msgs.len());
    }

    pub fn fire_weapon(&mut self) {
        let mut x = 0;
        let mut y = 0;
        if let Some(ship) = self.get_ship() {
            x = ship.get_x();
            y = ship.get_y();
        }

        self.entities.push(Entity { kind: EntityType::Laser, x: x, y: y, draw: true });
        let laser_id: EntityId = (self.entities.len() - 1) as EntityId;
        self.phys_manager.add_comp(laser_id, (0.0, 100.0));
    }

    fn get_ship(&self) -> Option<&Entity> {
        self.entities.get(0)
    }

    fn get_ship_mut(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(0)
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
            ship.set_x((x - width as i32 / 40).rem_euclid(width as i32));
        }
    }
    pub fn right_arrow_key(&mut self) {
        let width = self.width;
        if let Some(ship) = self.get_ship_mut() {
            let x = ship.get_x();
            ship.set_x((x + width as i32 / 40) % width as i32)
        }
    }
    pub fn up_arrow_key(&mut self) {
        let height = self.height;
        let max_ship = self.max_ship;
        if let Some(ship) = self.get_ship_mut() {
            let y = ship.get_y();
            ship.set_y(min(y + height as i32 / 40, max_ship as i32));
        }
    }
    pub fn down_arrow_key(&mut self) {
        let height = self.height;
        if let Some(ship) = self.get_ship_mut() {
            let y = ship.get_y();
            ship.set_y(max(y - height as i32 / 40, 0i32));
        }
    }
}
