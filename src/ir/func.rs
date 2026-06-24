use crate::{ecs::{EntityRef, map::{OwnedMap, RefMap}, option::{CompactOption, InvalidRepr}, set::{ExpandingRefSet, FixedRefSet}}, entity_impl, ir::{IrBuilder, KaliumBlock, KaliumBlockRef, inst::{KaliumInst, KaliumInstRef}, typ::KaliumType, value::{KaliumConstValue, KaliumImm, KaliumValue, KaliumValueRef}}};

pub struct KaliumFunc<'a> {
    pub(crate) ret_type: KaliumType,
    pub(crate) arg_types: &'a [KaliumType],
    pub(crate) insts: OwnedMap<KaliumInst, KaliumInstRef>,
    pub(crate) blocks: OwnedMap<KaliumBlock, KaliumBlockRef>,
    pub(crate) inst_results: RefMap<(KaliumValue, KaliumType), KaliumInstRef>,
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
        self.func.blocks.insert(KaliumBlock {
            insts: ExpandingRefSet::new(),
            predecessors: FixedRefSet::new()
        })
    }

    /// Set the active block. Returns true if the active block was set, false if it wasn't.
    pub fn select_block(&mut self, block: KaliumBlockRef) -> bool {
        if block.index() >= self.func.blocks.len() {
            return false
        }

        self.active_block = CompactOption::some(block);
        true
    }

    /// Create a signed integer value.
    ///
    /// Returns CompactOption::none if the type is not an integer type
    pub fn iconst(&mut self, imm: isize, typ: KaliumType) -> CompactOption<KaliumInstRef> {
        if !typ.is_integer() {
            return CompactOption::none();
        }

        let val = KaliumConstValue(KaliumImm::Int(imm));
        let inst = KaliumInst::ConstValue(val.clone());
        let inst_ref = self.func.insts.insert(inst);
        self.func.inst_results.insert((KaliumValue::Const(val), typ), inst_ref);

        CompactOption::some(inst_ref)
    }

    /// Create an unsigned integer value.
    ///
    /// Returns CompactOption::none if the type is not an integer type
    pub fn uconst(&mut self, imm: usize, typ: KaliumType) -> CompactOption<KaliumInstRef> {
        if !typ.is_integer() {
            return CompactOption::none();
        }

        let val = KaliumConstValue(KaliumImm::UInt(imm));
        let inst = KaliumInst::ConstValue(val.clone());
        let inst_ref = self.func.insts.insert(inst);
        self.func.inst_results.insert((KaliumValue::Const(val), typ), inst_ref);

        CompactOption::some(inst_ref)
    }

    #[inline]
    pub fn inst_result(&self, inst_ref: KaliumInstRef) -> CompactOption<KaliumValueRef> {
        if !self.func.inst_results.ref_is_valid(inst_ref) {
            return CompactOption::none()
        };

        // Value refs point to an instruction, like instruction refs, but they lookup the value returned by the instruction rather than the instruction itself.
        CompactOption::some(KaliumValueRef::new(inst_ref.index()))
    }

    fn val_to_inst_ref(val_ref: KaliumValueRef) -> KaliumInstRef {
        KaliumInstRef::new(val_ref.index())
    }

    /// Returns CompacOption::none if typ does not match the type of the imm or lhs, or if the value reference is invalid
    pub fn add_imm(&mut self, lhs: KaliumValueRef, imm: KaliumImm, typ: KaliumType) -> CompactOption<KaliumInstRef> {
        let (lhs_val, lhs_typ) = self.func.inst_results.get(Self::val_to_inst_ref(lhs)).unwrap();

        if *lhs_typ != typ {
            return CompactOption::none()
        }

        match lhs_val {
            KaliumValue::Const(KaliumConstValue(KaliumImm::Int(v))) if typ.is_signed() => {
                let KaliumImm::Int(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Int(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::UInt(v))) if typ.is_unsigned() => {
                let KaliumImm::UInt(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::UInt(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::Double(v))) if matches!(typ, KaliumType::Float64) => {
                let KaliumImm::Double(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Double(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::Float(v))) if matches!(typ, KaliumType::Float32) => {
                let KaliumImm::Float(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Float(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            // We do NOT know the lhs, so we must emit an instruction
            KaliumValue::Runtime => {
                let res = KaliumValue::Runtime;
                let inst = KaliumInst::AddImm(lhs, imm, typ);
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((res, typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            _ => CompactOption::none()
        }
    }

    /// Returns CompacOption::none if typ does not match the type of the imm or lhs, or if the value reference is invalid
    pub fn add(&mut self, lhs: KaliumValueRef, rhs: KaliumValueRef, typ: KaliumType) -> CompactOption<KaliumInstRef> {
        let (lhs_val, lhs_typ) = self.func.inst_results.get(Self::val_to_inst_ref(lhs)).unwrap();
        let (rhs_val, rhs_typ) = self.func.inst_results.get(Self::val_to_inst_ref(rhs)).unwrap();

        if *lhs_typ != typ {
            return CompactOption::none()
        }

        match lhs_val {
            KaliumValue::Const(KaliumConstValue(KaliumImm::Int(v))) if typ.is_signed() => {
                let KaliumImm::Int(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Int(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::UInt(v))) if typ.is_unsigned() => {
                let KaliumImm::UInt(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::UInt(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::Double(v))) if matches!(typ, KaliumType::Float64) => {
                let KaliumImm::Double(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Double(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            KaliumValue::Const(KaliumConstValue(KaliumImm::Float(v))) if matches!(typ, KaliumType::Float32) => {
                let KaliumImm::Float(rhs) = imm else {
                    return CompactOption::none()
                };

                let res = KaliumConstValue(KaliumImm::Float(v + rhs));

                let inst = KaliumInst::ConstValue(res.clone());
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((KaliumValue::Const(res), typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            // We do NOT know the lhs, so we must emit an instruction
            KaliumValue::Runtime => {
                let res = KaliumValue::Runtime;
                let inst = KaliumInst::AddImm(lhs, imm, typ);
                let inst_ref = self.func.insts.insert(inst);

                let active_block = self.func.blocks.get_mut(self.active_block.unwrap()).unwrap();

                active_block.insts.insert(inst_ref);
                self.func.inst_results.insert((res, typ), inst_ref);

                CompactOption::some(inst_ref)
            }
            _ => CompactOption::none()
        }
    }
}