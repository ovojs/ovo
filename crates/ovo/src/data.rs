use ovo_quickjs::JSValue;
use std::convert::From;
use std::mem::transmute;

#[repr(C)]
pub struct Value(pub(crate) JSValue);

#[repr(C)]
pub struct Bool(pub(crate) JSValue);

#[repr(C)]
pub struct Int32(pub(crate) JSValue);

#[repr(C)]
pub struct Uint32(pub(crate) JSValue);

#[repr(C)]
pub struct Int64(pub(crate) JSValue);

#[repr(C)]
pub struct BigInt64(pub(crate) JSValue);

#[repr(C)]
pub struct BigUint64(pub(crate) JSValue);

#[repr(C)]
pub struct Float64(pub(crate) JSValue);

#[repr(C)]
pub struct String(pub(crate) JSValue);

macro_rules! impl_from {
  ($source:ident for $type:ident) => {
    impl From<$source> for $type {
      fn from(value: $source) -> Self {
        unsafe { transmute(value) }
      }
    }
  };
}

impl_from!(Bool for Value);
impl_from!(Int32 for Value);
impl_from!(Uint32 for Value);
impl_from!(Int64 for Value);
impl_from!(BigInt64 for Value);
impl_from!(BigUint64 for Value);
impl_from!(Float64 for Value);
impl_from!(String for Value);

impl_from!(Value for JSValue);
