use crate::{ecs::{EntityRef, option::{CompactOption, InvalidRepr}, set::ExpandingRefSet}, entity_impl, ir::{IrBuilder, KaliumBlockRef, typ::KaliumType, value::{KaliumConstValue, KaliumImm, KaliumValue, KaliumValueRef}}};

pub struct KaliumFunc<'a> {
    ret_type: KaliumType,
    arg_types: &'a [KaliumType],
    blocks: ExpandingRefSet<KaliumBlockRef, u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KaliumFuncRef(u32);
entity_impl!(KaliumFuncRef : u32);

pub struct FuncBuilder<'a> {
    func: &'a mut KaliumFunc<'a>,
    ir_builder: &'a mut IrBuilder,
    name: &'a str,
    ret_type: KaliumType,
    arg_types: &'a [KaliumType],
    active_block: CompactOption<KaliumBlockRef>
}

impl<'a> FuncBuilder<'a> {
    pub fn new(
        func: &'a mut KaliumFunc<'a>,
        ir_builder: &'a mut IrBuilder,
        name: &'a str,
        ret_type: KaliumType,
        arg_types: &'a [KaliumType],
    ) -> Self {
        Self {
            func,
            ir_builder,
            name,
            ret_type,
            arg_types,
            active_block: CompactOption::none()
        }
    }

    pub fn create_block(&mut self) -> KaliumBlockRef {
        self.ir_builder.add_block()
    }

    /// Set the active block. Returns true if the active block was set, false if it wasn't.
    pub fn select_block(&mut self, block: KaliumBlockRef) -> bool {
        if !self.ir_builder.blocks().ref_is_valid(block) {
            return false
        }

        self.active_block = CompactOption::some(block);
        true
    }

    /// Create a signed integer value.
    ///
    /// Returns CompactOption::none if the type is not an integer type
    pub fn iconst(&mut self, imm: isize, typ: KaliumType) -> CompactOption<KaliumValueRef> {
        if !typ.is_integer() {
            return CompactOption::none();
        }

        let val = KaliumConstValue(KaliumImm::Int(imm));
        self.ir_builder.inst_results_mut().push(KaliumValue::Const(val));

        let eref = KaliumValueRef::new(self.ir_builder.inst_results().len() - 1);
        CompactOption::some(eref)
    }

    /// Create an unsigned integer value.
    ///
    /// Returns CompactOption::none if the type is not an integer type
    pub fn uconst(&mut self, imm: usize, typ: KaliumType) -> CompactOption<KaliumValueRef> {
        if !typ.is_integer() {
            return CompactOption::none();
        }

        let val = KaliumConstValue(KaliumImm::UInt(imm));
        let eref = self.ir_builder.inst_results_mut().insert(KaliumValue::Const(val));
        CompactOption::some(eref)
    }

    /// Returns CompacOption::none if typ does not match the type of the imm or lhs, or if the value reference is invalid
    pub fn add_imm(&mut self, lhs: KaliumValueRef, imm: KaliumImm, typ: KaliumType) -> CompactOption<KaliumValueRef> {
        let Some(typ) = self.ir_builder.value_types().get(lhs) else {
            return CompactOption::none()
        };



        let inst = KaliumInst::AddImm(lhs, imm, ())

        let active_block = self.ir_builder.blocks_mut().get(self.active_block.unwrap()).unwrap();
        active_block.insts
    }
}