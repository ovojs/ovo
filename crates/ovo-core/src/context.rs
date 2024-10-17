use crate::handle::Owned;
use crate::quickjs::*;
use crate::runtime::Runtime;
use crate::value::{String, Value};
use anyhow::{anyhow, Error};
use std::ffi::c_int;
use std::mem::transmute;
use std::ptr::NonNull;

pub struct Context(pub(crate) NonNull<JSContext>);

impl Context {
  pub fn new(rt: &Runtime) -> Self {
    let raw_rt = rt.inner.as_ptr();
    let raw_ctx = unsafe { JS_NewContext(raw_rt) };
    Self(NonNull::new(raw_ctx).expect("non-null context"))
  }

  pub fn from_raw(raw: *mut JSContext) -> Self {
    Self(NonNull::new(raw).expect("non-null context"))
  }

  pub fn compile(
    &self,
    source: String,
    referer: String,
    is_module: bool,
  ) -> Result<Owned<Value>, Error> {
    let eval_type = if is_module {
      EvalType::Module(EvalFlag::CompileOnly)
    } else {
      EvalType::Script(EvalFlag::CompileOnly)
    };
    self.evaluate(source, referer, eval_type)
  }

  pub fn evaluate(
    &self,
    source: String,
    referer: String,
    eval_type: EvalType,
  ) -> Result<Owned<Value>, Error> {
    let source = source.value(self);
    let referer = referer.value(self);
    let flags = eval_type.to_flags();
    let value = Value(unsafe {
      JS_Eval(
        self.0.as_ptr(),
        source.as_ptr() as *const i8,
        source.len(),
        referer.as_ptr() as *const i8,
        flags as c_int,
      )
    });
    self.to_owned_or_error(value)
  }

  pub fn evaluate_function(&self, func: Value) -> Result<Owned<Value>, Error> {
    let value = Value(unsafe { JS_EvalFunction(self.0.as_ptr(), func.into()) });
    self.to_owned_or_error(value)
  }

  fn to_owned_or_error(&self, value: Value) -> Result<Owned<Value>, Error> {
    if value.is_exception() {
      Err(anyhow!("todo: get exception message from value"))
    } else {
      Ok(Owned::new(self.clone(), value))
    }
  }
}

impl Drop for Context {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { JS_FreeContext(self.0.as_ptr()) }
  }
}

impl Clone for Context {
  fn clone(&self) -> Self {
    let self_raw = self.0.as_ptr();
    let inner = match NonNull::new(unsafe { JS_DupContext(self_raw) }) {
      Some(inner) => inner,
      None => panic!("null context"),
    };
    Context(inner)
  }
}

pub enum EvalFlag {
  None = 0,
  Async = JS_EVAL_FLAG_ASYNC as isize,
  CompileOnly = JS_EVAL_FLAG_COMPILE_ONLY as isize,
  BacktraceBarrier = JS_EVAL_FLAG_BACKTRACE_BARRIER as isize,
  Strict = JS_EVAL_FLAG_STRICT as isize,
  Strip = JS_EVAL_FLAG_STRIP as isize,
}

impl EvalFlag {
  fn into_u8(self) -> u8 {
    unsafe { transmute(self) }
  }
}

pub enum EvalType {
  Script(EvalFlag),
  Module(EvalFlag),
}

impl EvalType {
  fn to_flags(self) -> u32 {
    match self {
      Self::Script(flags) => JS_EVAL_TYPE_GLOBAL | flags.into_u8() as u32,
      Self::Module(flags) => JS_EVAL_TYPE_MODULE | flags.into_u8() as u32,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::runtime::{Runtime, RuntimeOptions};
  use crate::value::Int32;

  #[test]
  fn evaluate_script() {
    let rt = Runtime::new(RuntimeOptions::default());
    let ctx = Context::new(&rt);
    let value = ctx
      .evaluate(
        String::new(&ctx, "40 + 2"),
        String::new(&ctx, "init"),
        EvalType::Script(EvalFlag::None),
      )
      .expect("42");
    let expected = Value::from(Int32::new(&ctx, 42));
    assert!(value == expected);
    assert!(value == Owned::new(ctx, expected));
  }
}
