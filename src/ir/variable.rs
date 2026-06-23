use crate::{ecs::EntityRef, entity_impl, ir::{typ::KaliumType, value::KaliumValueRef}};

pub struct KaliumVariable {
    typ: KaliumType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumVariableRef(u32);
entity_impl!(KaliumVariableRef : u32);