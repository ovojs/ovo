use crate::{BigInt64, BigUint64, Bool, Int32, Int64, String, Uint32};
use crate::{Context, Float64};
use ovo_quickjs::{
  JS_NewBigInt64, JS_NewBigUint64, JS_NewBool, JS_NewFloat64, JS_NewInt32,
  JS_NewInt64, JS_NewStringLen, JS_NewUint32, JS_ToBigInt64, JS_ToBool,
  JS_ToCString, JS_ToFloat64, JS_ToInt32, JS_ToInt64, JS_ToString, JS_ToUint32,
};
use std::ffi::{c_char, c_int, CStr};

impl Bool {
  pub fn new(ctx: &Context, value: bool) -> Self {
    unsafe { Self(JS_NewBool(ctx.inner.as_ptr(), value as c_int)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<bool> {
    unsafe { Some(JS_ToBool(ctx.inner.as_ptr(), self.0) != 0) }
  }
}

impl Int32 {
  pub fn new(ctx: &Context, value: i32) -> Self {
    unsafe { Self(JS_NewInt32(ctx.inner.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<i32> {
    let mut value = 0;
    let ret =
      unsafe { JS_ToInt32(ctx.inner.as_ptr(), &mut value as *mut i32, self.0) };
    if ret == 0 {
      Some(value)
    } else {
      None
    }
  }
}

impl Uint32 {
  pub fn new(ctx: &mut Context, value: u32) -> Self {
    unsafe { Self(JS_NewUint32(ctx.inner.as_mut(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<u32> {
    let mut value = 0;
    let ret = unsafe {
      JS_ToUint32(ctx.inner.as_ptr(), &mut value as *mut u32, self.0)
    };
    if ret == 0 {
      Some(value)
    } else {
      None
    }
  }
}

impl Int64 {
  pub fn new(ctx: &Context, value: i64) -> Self {
    unsafe { Self(JS_NewInt64(ctx.inner.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<i64> {
    let mut value = 0;
    let ret =
      unsafe { JS_ToInt64(ctx.inner.as_ptr(), &mut value as *mut i64, self.0) };
    if ret == 0 {
      Some(value)
    } else {
      None
    }
  }
}

impl BigUint64 {
  pub fn new(ctx: &Context, value: u64) -> Self {
    unsafe { Self(JS_NewBigUint64(ctx.inner.as_ptr(), value)) }
  }

  pub fn value(&self, _: &Context) -> Option<u64> {
    unimplemented!()
  }
}

impl BigInt64 {
  pub fn new(ctx: &Context, value: i64) -> Self {
    unsafe { Self(JS_NewBigInt64(ctx.inner.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<i64> {
    let mut value = 0;
    let ret = unsafe {
      JS_ToBigInt64(ctx.inner.as_ptr(), &mut value as *mut i64, self.0)
    };
    if ret == 0 {
      Some(value)
    } else {
      None
    }
  }
}

impl Float64 {
  pub fn new(ctx: &Context, value: f64) -> Self {
    unsafe { Self(JS_NewFloat64(ctx.inner.as_ptr(), value)) }
  }

  pub fn value(&self, ctx: &Context) -> Option<f64> {
    let mut value = 0.;
    let ret = unsafe {
      JS_ToFloat64(ctx.inner.as_ptr(), &mut value as *mut f64, self.0)
    };
    if ret == 0 {
      Some(value)
    } else {
      None
    }
  }
}

impl String {
  pub fn new(ctx: &Context, value: &str) -> Self {
    unsafe {
      Self(JS_NewStringLen(
        ctx.inner.as_ptr(),
        value.as_ptr() as *const c_char,
        value.len(),
      ))
    }
  }

  pub fn into_str(&self, ctx: &Context) -> &str {
    unsafe {
      CStr::from_ptr(JS_ToCString(ctx.inner.as_ptr(), self.0))
        .to_str()
        .unwrap()
    }
  }
}
