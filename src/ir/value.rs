use crate::{ecs::EntityRef, entity_impl, ir::typ::KaliumType};

/// A value that is either a KaliumConstValue or only known at runtime
#[derive(Clone)]
pub enum KaliumValue {
    Const(KaliumConstValue),
    Runtime,
}

/// A value whose value is known at compile time. This differs from an immediate value as you can obtain a KaliumValueRef to a KaliumConstValue
#[derive(Clone)]
pub struct KaliumConstValue(pub KaliumImm);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumValueRef(u32);
entity_impl!(KaliumValueRef : u32);

#[derive(Clone)]
pub enum KaliumImm {
    UInt(usize),
    Int(isize),
    Float(f32),
    Double(f64),
}

/// A value that exists for the entire duration of the program, with its memory reserved in .bss
pub struct KaliumStaticValue {
    size: u32,
    min_align_shift: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumStaticValueRef(u32);
entity_impl!(KaliumStaticValueRef : u32);

/// A global constant
pub struct KaliumGlobalValue {
    size: u32,
    min_align_shift: u8,
    data: Box<[u8]>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumGlobalValueRef(u32);
entity_impl!(KaliumGlobalValueRef : u32);