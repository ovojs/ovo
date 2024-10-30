use crate::context::Context;
use crate::ffi::*;
use crate::value::Value;
use std::ffi::c_int;

pub trait Op {
  const NAME: &'static str;
  const DECL: OpDecl;
}

pub struct OpDecl {
  narg: u8,
  r#fn: JSCFunction,
}

impl OpDecl {
  pub const fn new(narg: u8, r#fn: JSCFunction) -> Self {
    Self { narg, r#fn }
  }
}

pub struct CallScope {
  ctx: Context,
  argc: c_int,
  argv: *mut JSValue,
  this_val: JSValue,
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
      argc,
      argv,
      this_val,
    }
  }

  #[inline(always)]
  pub fn context(&self) -> &Context {
    &self.ctx
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
