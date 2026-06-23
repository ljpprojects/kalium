use crate::entity_impl;

pub struct KaliumStackSlot {
    size: u32,
    /// The minimum alignment for the stack slot. In practice, values lower than 4 will be ignored as the stack pointer must maintain 16-byte alignment on most platforms.
    min_align_shift: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumStackSlotRef(u32);
entity_impl!(KaliumStackSlotRef : u32);