use wasm_bindgen::prelude::*;

use crate::log;
use crate::common::{ Vec2, Entity, EntityId };
use crate::msg::{ Msg };

enum EdgeBehavior {
    Die
}

struct Comp {
    velocity: Vec2,
    id: EntityId,
    edge_behavior: EdgeBehavior,
    active: bool,
}

#[wasm_bindgen]
pub struct PhysicsManager {
    comps: Vec<Comp>
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        PhysicsManager {
            comps: vec![]
        }
    }

    pub fn add_comp(&mut self, id: EntityId, vel: Vec2) {
        self.comps.push(Comp {
            id: id,
            velocity: vel,
            edge_behavior: EdgeBehavior::Die,
            active: true
        });
    }

    pub fn tick(&mut self, dt: u32, entities: &mut Vec<Entity>, current_msgs: &Vec<Msg>, max_x: u32, max_y: u32) -> Vec<Msg> {
        let mut next_msgs = Vec::new();

        for msg in current_msgs {
            log!("{:?}", msg);
            match msg {
                Msg::DeleteEntity(entity) => {
                    let mut id = None;
                    for (i, comp) in self.comps.iter().enumerate() {
                        if comp.id == *entity {
                            id = Some(i);
                        }
                    }

                    if let Some(i) = id {
                        self.comps[i].active = false;
                    }
                }
            }
        }

        for comp in &self.comps {
            if !comp.active {
                continue
            }

            let dx = comp.velocity.0 * dt as f32 / 100.0;
            let dy = comp.velocity.1 * dt as f32 / 100.0;
 
            if let Some(e) = entities.get_mut(comp.id as usize) {
                let x = (e.get_x() as f32 + dx).round() as i32;
                let y = (e.get_y() as f32 + dy).round() as i32;
                e.set_x(x);
                e.set_y(y);

                if x > max_x as i32 || y > max_y as i32 {
                    match &comp.edge_behavior {
                        EdgeBehavior::Die => {
                            next_msgs.push(Msg::DeleteEntity(comp.id))
                        }
                    }
                }
            }
        }

        return next_msgs;
    }
}
