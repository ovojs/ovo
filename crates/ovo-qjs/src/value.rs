use crate::context::Context;
use crate::ffi::*;
use crate::handle::{CloneFromContext, DropFromContext, PartialEqFromContext};
use std::convert::From;
use std::ffi::{c_char, c_int, CStr};
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

#[repr(C)]
pub struct Symbol(pub(crate) JSValue);

#[repr(C)]
pub struct Object(pub(crate) JSValue);

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
impl_from!(Symbol for Value);
impl_from!(Object for Value);

impl_from!(Value for JSValue);

macro_rules! try_value_some {
  ($ctx:ident, $value:ident, $type:ident, $is:ident) => {
    if $value.$is() {
      let v: &crate::value::$type = unsafe { std::mem::transmute($value) };
      return Some(v.value($ctx));
    }
  };
}

impl Value {
  pub fn from_js_value(raw: JSValue) -> Self {
    Self(raw)
  }

  #[inline(always)]
  pub fn is_exception(&self) -> bool {
    unsafe { JS_IsException(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_null(&self) -> bool {
    unsafe { JS_IsNull(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_undefined(&self) -> bool {
    unsafe { JS_IsUndefined(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_bool(&self) -> bool {
    unsafe { JS_IsBool(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_number(&self) -> bool {
    unsafe { JS_IsNumber(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_string(&self) -> bool {
    unsafe { JS_IsString(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_symbol(&self) -> bool {
    unsafe { JS_IsSymbol(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_object(&self) -> bool {
    unsafe { JS_IsObject(self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_error(&self, ctx: &Context) -> bool {
    unsafe { JS_IsError(ctx.0.as_ptr(), self.0) != 0 }
  }

  #[inline(always)]
  pub fn is_array(&self, ctx: &Context) -> bool {
    unsafe { JS_IsArray(ctx.0.as_ptr(), self.0) != 0 }
  }

  pub fn try_to_i32(&self, ctx: &Context) -> Option<i32> {
    try_value_some!(ctx, self, Int32, is_number);
    None
  }

  pub fn try_to_u32(&self, ctx: &Context) -> Option<u32> {
    try_value_some!(ctx, self, Uint32, is_number);
    None
  }

  pub fn try_to_bool(&self, ctx: &Context) -> Option<bool> {
    try_value_some!(ctx, self, Bool, is_bool);
    None
  }

  pub fn try_to_string(&self, ctx: &Context) -> Option<std::string::String> {
    if self.is_string() {
      let v: &String = unsafe { std::mem::transmute(self) };
      Some(v.value(ctx).to_string())
    } else {
      None
    }
  }
}

impl DropFromContext for Value {
  fn drop_from_context(&mut self, ctx: &Context) {
    unsafe { JS_FreeValue(ctx.0.as_ptr(), self.0) };
  }
}

impl CloneFromContext for Value {
  fn clone_from_context(&self, ctx: &Context) -> Self {
    unsafe { Self(JS_DupValue(ctx.0.as_ptr(), self.0)) }
  }
}

impl PartialEqFromContext for Value {
  fn eq_from_context(&self, ctx: &Context, other: &Self) -> bool {
    unsafe { JS_IsStrictEqual(ctx.0.as_ptr(), self.0, other.0) != 0 }
  }
}

impl Bool {
  pub fn new(ctx: &Context, value: bool) -> Self {
    unsafe { Self(JS_NewBool(ctx.0.as_ptr(), value as c_int)) }
  }

  pub fn value(&self, ctx: &Context) -> bool {
    unsafe { JS_ToBool(ctx.0.as_ptr(), self.0) != 0 }
  }
}

impl Int32 {
  pub fn new(ctx: &Context, value: i32) -> Self {
    unsafe { Self(JS_NewInt32(ctx.0.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> i32 {
    unsafe {
      let mut value = 0;
      JS_ToInt32(ctx.0.as_ptr(), &mut value as *mut i32, self.0);
      value
    }
  }
}

impl Uint32 {
  pub fn new(ctx: &mut Context, value: u32) -> Self {
    unsafe { Self(JS_NewUint32(ctx.0.as_mut(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> u32 {
    unsafe {
      let mut value = 0;
      JS_ToUint32(ctx.0.as_ptr(), &mut value as *mut u32, self.0);
      value
    }
  }
}

impl Int64 {
  pub fn new(ctx: &Context, value: i64) -> Self {
    unsafe { Self(JS_NewInt64(ctx.0.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> i64 {
    unsafe {
      let mut value = 0;
      JS_ToInt64(ctx.0.as_ptr(), &mut value as *mut i64, self.0);
      value
    }
  }
}

impl BigUint64 {
  pub fn new(ctx: &Context, value: u64) -> Self {
    unsafe { Self(JS_NewBigUint64(ctx.0.as_ptr(), value)) }
  }

  pub fn value(&self, _: &Context) -> u64 {
    unimplemented!()
  }
}

impl BigInt64 {
  pub fn new(ctx: &Context, value: i64) -> Self {
    unsafe { Self(JS_NewBigInt64(ctx.0.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> i64 {
    unsafe {
      let mut value = 0;
      JS_ToBigInt64(ctx.0.as_ptr(), &mut value as *mut i64, self.0);
      value
    }
  }
}

impl Float64 {
  pub fn new(ctx: &Context, value: f64) -> Self {
    unsafe { Self(JS_NewFloat64(ctx.0.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> f64 {
    unsafe {
      let mut value = 0.;
      JS_ToFloat64(ctx.0.as_ptr(), &mut value as *mut f64, self.0);
      value
    }
  }
}

impl String {
  pub fn new(ctx: &Context, value: &str) -> Self {
    unsafe {
      Self(JS_NewStringLen(
        ctx.0.as_ptr(),
        value.as_ptr() as *const c_char,
        value.len(),
      ))
    }
  }

  pub fn value(&self, ctx: &Context) -> &str {
    unsafe {
      CStr::from_ptr(JS_ToCString(ctx.0.as_ptr(), self.0))
        .to_str()
        .unwrap()
    }
  }
}

impl Symbol {
  pub fn new(ctx: &Context, name: &'static str, is_global: bool) -> Self {
    unsafe {
      Self(JS_NewSymbol(
        ctx.0.as_ptr(),
        name.as_ptr() as *const i8,
        is_global as i32,
      ))
    }
  }
}

impl Object {
  pub fn new(ctx: &Context) -> Self {
    let inner = unsafe { JS_NewObject(ctx.0.as_ptr()) };
    Self(inner)
  }
}

#[cfg(test)]
mod tests {
  use crate::context::Context;
  use crate::runtime::{Runtime, RuntimeOptions};
  use crate::value::{BigInt64, Bool, Float64, Int32, Int64, String};
  use std::i64;

  #[test]
  fn value_assertion() {
    let runtime = &Runtime::new(RuntimeOptions::default());
    let context = &Context::new(runtime);

    let value = true;
    let boolean = Bool::new(context, true);
    assert_eq!(boolean.value(context), value);

    let value = 114514;
    let int32 = Int32::new(context, value);
    assert_eq!(int32.value(context), value);

    let value = 114.514;
    let float64 = Float64::new(context, value);
    assert_eq!(float64.value(context), value);

    let value = 114514;
    let int64 = Int64::new(context, value);
    assert_eq!(int64.value(context), value);

    let value = i64::MAX;
    let bigint64 = BigInt64::new(context, value);
    assert_eq!(bigint64.value(context), value);

    let value = "OvO";
    let string = String::new(context, value);
    assert_eq!(string.value(context), value);
  }
}
