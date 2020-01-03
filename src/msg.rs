use crate::common::{ EntityId };

#[derive(Debug)]
pub enum Msg {
    DeleteEntity(EntityId)
}
