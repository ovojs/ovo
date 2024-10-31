use crate::context::Context;
use crate::ffi::*;
use crate::value::Value;
use std::borrow::Cow;
use std::ffi::c_int;

#[derive(Clone, Copy)]
pub struct Op {
  pub name: &'static str,
  pub r#fn: JSCFunction,
}

pub struct Ext {
  pub name: &'static str,
  pub ops: Cow<'static, [Op]>,
}

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
  pub fn this(&self) -> &Value {
    &self.this_val
  }

  #[inline(always)]
  pub fn get(&self, i: usize) -> Value {
    unsafe { Value::from_js_value(*self.argv.add(i)) }
  }
}

#[macro_export]
macro_rules! ext {
  ($name:ident $(, ops = [ $($op:ident)+ ])?) => {};
}
