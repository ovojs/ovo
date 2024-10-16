use crate::context::Context;
use crate::quickjs::*;
use crate::value::{
  BigInt64, BigUint64, Bool, Float64, Int32, Int64, String, Uint32,
};
use std::ffi::{c_char, c_int, CStr};

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
