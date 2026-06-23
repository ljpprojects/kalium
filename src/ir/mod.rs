use crate::{ecs::{EntityRef, map::{OwnedMap, RefMap}, option::{CompactOption, InvalidRepr}, set::{ExpandingRefSet, FixedRefSet}}, entity_impl, ir::{inst::{KaliumInst, KaliumInstRef}, typ::KaliumType, value::{KaliumValue, KaliumValueRef}, variable::{KaliumVariable, KaliumVariableRef}}};

pub mod func;
pub mod inst;
pub mod stack;
pub mod typ;
pub mod value;
pub mod variable;

pub struct KaliumBlock {
    insts: Vec<KaliumInst>,
    predecessors: FixedRefSet<KaliumBlockRef, u64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumBlockRef(u32);
entity_impl!(KaliumBlockRef : u32);

pub struct IrBuilder {
    variables: OwnedMap<KaliumVariable, KaliumVariableRef>,
    variable_info: RefMap<CompactOption<KaliumBlockRef>, KaliumVariableRef>,
    inst_results: OwnedMap<KaliumValue, KaliumValueRef>,
    value_types: RefMap<KaliumType, KaliumValueRef>,
    blocks: OwnedMap<KaliumBlock, KaliumBlockRef>,
    visited_blocks: ExpandingRefSet<KaliumBlockRef, u64>,
}

impl IrBuilder {
    pub fn add_block(&mut self) -> KaliumBlockRef {
        self.blocks.insert(KaliumBlock {
            insts: Vec::new(),
            predecessors: FixedRefSet::new(),
        })
    }

    pub fn blocks<'a>(&'a self) -> &'a OwnedMap<KaliumBlock, KaliumBlockRef> {
        &self.blocks
    }

    pub(crate) fn blocks_mut<'a>(&'a mut self) -> &'a mut OwnedMap<KaliumBlock, KaliumBlockRef> {
        &mut self.blocks
    }

    pub(crate) fn inst_results<'a>(&'a self) -> &'a OwnedMap<KaliumValue, KaliumValueRef> {
        &self.inst_results
    }

    pub(crate) fn inst_results_mut<'a>(&'a mut self) -> &'a mut OwnedMap<KaliumValue, KaliumValueRef> {
        &mut self.inst_results
    }

    pub(crate) fn value_types<'a>(&'a self) -> &'a RefMap<KaliumType, KaliumValueRef> {
        &self.value_types
    }

    pub(crate) fn value_types_mut<'a>(&'a mut self) -> &'a mut RefMap<KaliumType, KaliumValueRef> {
        &mut self.value_types
    }
}