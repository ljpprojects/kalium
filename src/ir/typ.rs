use std::ops::Deref;

use crate::ecs::option::{CompactOption, InvalidRepr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaliumType {
    Invalid,

    Int8,
    Int16,
    Int32,
    Int64,
    IntPtr,

    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UIntPtr,

    Float32,
    Float64,

    // TODO: vectors?
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct KaliumTypeWidth(i32);

impl Deref for KaliumTypeWidth {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InvalidRepr for KaliumTypeWidth {
    fn is_valid(self) -> bool {
        *self > 0
    }

    unsafe fn default_invalid() -> Self {
        Self(-1)
    }
}

impl KaliumType {
    #[inline]
    pub fn is_integer(self) -> bool {
        !matches!(self, KaliumType::Invalid | KaliumType::Float32 | KaliumType::Float64)
    }

    #[inline]
    pub fn is_float(self) -> bool {
        matches!(self, KaliumType::Float32 | KaliumType::Float64)
    }

    #[inline]
    pub fn to_unsigned(self) -> CompactOption<Self> {
        match self {
            Self::UInt8 | Self::UInt16 | Self::UInt32 | Self::UInt64 | Self::UIntPtr => CompactOption::some(self),
            Self::Int8 => CompactOption::some(Self::UInt8),
            Self::Int16 => CompactOption::some(Self::UInt16),
            Self::Int32 => CompactOption::some(Self::UInt32),
            Self::Int64 => CompactOption::some(Self::UInt64),
            Self::IntPtr => CompactOption::some(Self::UIntPtr),
            _ => CompactOption::none(),
        }
    }

    #[inline]
    pub fn to_signed(self) -> CompactOption<Self> {
        match self {
            Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64 | Self::IntPtr => CompactOption::some(self),
            Self::UInt8 => CompactOption::some(Self::Int8),
            Self::UInt16 => CompactOption::some(Self::Int16),
            Self::UInt32 => CompactOption::some(Self::Int32),
            Self::UInt64 => CompactOption::some(Self::Int64),
            Self::UIntPtr => CompactOption::some(Self::IntPtr),
            _ => CompactOption::none(),
        }
    }

    #[inline]
    pub fn is_signed(self) -> bool {
        matches!(self, Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64 | Self::IntPtr)
    }

    #[inline]
    pub fn is_unsigned(self) -> bool {
        !self.is_signed()
    }

    /// Get the width of the type in bytes. Returns none if the type is Invalid or thw width is dependent on the target architecture. KaliumTypeWidth can be dereferenced as an i32.
    pub fn width(self) -> CompactOption<KaliumTypeWidth> {
        match self {
            Self::Int8 | Self::UInt8 => CompactOption::some(KaliumTypeWidth(1)),
            Self::Int16 | Self::UInt16 => CompactOption::some(KaliumTypeWidth(2)),
            Self::Int32 | Self::UInt32 | Self::Float32 => CompactOption::some(KaliumTypeWidth(4)),
            Self::Int64 | Self::UInt64 | Self::Float64 => CompactOption::some(KaliumTypeWidth(8)),
            Self::IntPtr | Self::UIntPtr => CompactOption::none(),
            Self::Invalid => CompactOption::none(),
        }
    }
}

impl InvalidRepr for KaliumType {
    fn is_valid(self) -> bool {
        !matches!(self, KaliumType::Invalid)
    }

    unsafe fn default_invalid() -> Self {
        KaliumType::Invalid
    }
}