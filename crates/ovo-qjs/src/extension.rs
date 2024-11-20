use crate::context::Context;
use crate::ffi::*;
use crate::value::Value;
use std::borrow::{Borrow, Cow};
use std::ffi::c_int;

pub type Fn = fn(&Context, CallArgs) -> Value;

#[derive(Clone, Copy)]
pub struct Op {
  pub name: &'static str,
  pub r#fn: JSCFunction,
  pub argc: u8,
  pub magic: i16,
}

pub struct Ext {
  pub name: &'static str,
  pub ops: Cow<'static, [Op]>,
}

impl Ext {}

pub struct CallArgs {
  this_val: Value,
  argc: u8,
  argv: *mut JSValue,
}

impl CallArgs {
  pub fn new(this_val: JSValue, argc: c_int, argv: *mut JSValue) -> Self {
    Self {
      this_val: Value::from_js_value(this_val),
      argc: argc as u8,
      argv,
    }
  }

  #[inline(always)]
  pub fn this_ref(&self) -> &Value {
    self.this_val.borrow()
  }

  #[inline(always)]
  pub fn get(&self, idx: u8) -> Value {
    if idx > self.argc {
      panic!("argument index out of range")
    }
    unsafe { Value::from_js_value(*self.argv.add(idx as usize)) }
  }

  #[inline(always)]
  pub fn len(&self) -> u8 {
    self.argc
  }
}

#[macro_export]
macro_rules! ext {
  ($name:ident $(, ops = [ $($op:ident)+ ])?) => {};
}

#[macro_export]
macro_rules! function {
  ($name:ident $f:expr) => {
    unsafe extern "C" fn $name(
      ctx: *mut $crate::ffi::JSContext,
      this_val: $crate::ffi::JSValue,
      argc: std::ffi::c_int,
      argv: *mut $crate::ffi::JSValue,
    ) -> $crate::ffi::JSValue {
      let ctx = $crate::context::Context::from(ctx);
      let args = $crate::extension::CallArgs::new(this_val, argc, argv);
      $crate::value::Value::from($f(&ctx, args)).into()
    }
  };
}
