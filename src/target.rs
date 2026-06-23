use std::ops::Deref;

use crate::ecs::option::InvalidRepr;

pub enum KaliumTargetOs {
    Darwin = 0x1,
    Linux = 0x2,
}

pub enum KaliumTargetArch {
    Aarch64 = 0x4,
    X86_64 = 0x8,
}

/// An enum with variants for each supported target
pub struct KaliumTarget {
    os: KaliumTargetOs,
    arch: KaliumTargetArch
}

#[derive(Clone, Copy)]
pub struct KaliumTargetSupport(pub u32);

impl KaliumTargetSupport {
    pub const fn none() -> Self {
        KaliumTargetSupport(0)
    }

    pub const fn all() -> Self {
        KaliumTargetSupport(0xEFFF_FFFF)
    }

    pub fn and_os(self, os: KaliumTargetOs) -> Self {
        KaliumTargetSupport(self.0 & ((os as u32) << 1))
    }

    pub fn and_arch(self, arch: KaliumTargetArch) -> Self {
        KaliumTargetSupport(self.0 & ((arch as u32) << 1))
    }

    pub fn allows_target(&self, target: KaliumTarget) -> bool {
        self.0 & (target.os as u32) != 0 && self.0 & (target.arch as u32) != 0
    }
}

impl InvalidRepr for KaliumTargetSupport {
    fn is_valid(self) -> bool {
        self.0 != 0xFFFF_FFFF
    }

    unsafe fn default_invalid() -> Self {
        Self(0xFFFF_FFFF)
    }
}

impl Deref for KaliumTargetSupport {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}