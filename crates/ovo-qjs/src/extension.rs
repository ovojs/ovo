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

pub struct CallScope {
  ctx: Context,
  this_val: Value,
  argc: c_int,
  argv: *mut JSValue,
}

impl CallScope {
  pub fn new(
    ctx: *mut JSContext,
    this_val: JSValue,
    argc: c_int,
    argv: *mut JSValue,
  ) -> Self {
    Self {
      ctx: Context::from_raw(ctx),
      this_val: Value::from_js_value(this_val),
      argc,
      argv,
    }
  }

  #[inline(always)]
  pub fn context(&self) -> &Context {
    &self.ctx
  }

  #[inline(always)]
  pub fn this_value(&self) -> &Value {
    &self.this_val
  }

  #[inline(always)]
  pub fn get(&self, i: usize) -> Value {
    unsafe { Value::from_js_value(*self.argv.add(i)) }
  }

  #[inline(always)]
  pub fn throw_type_error(&self, msg: &str) -> JSValue {
    unsafe { JS_ThrowTypeError(self.ctx.0.as_ptr(), msg.as_ptr() as *const i8) }
  }
}

#[macro_export]
macro_rules! ext {
  ($name:ident $(, ops = [ $($op:ident)+ ])?) => {};
}
