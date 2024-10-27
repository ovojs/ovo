use crate::ffi::*;
use crate::handle::Owned;
use crate::runtime::Runtime;
use crate::source::{Flag, Source};
use crate::value::Value;
use std::ffi::{c_char, c_int};
use std::ptr::NonNull;

pub struct Context(pub(crate) NonNull<JSContext>);

impl Context {
  pub fn new(runtime: &Runtime) -> Self {
    let raw_runtime = runtime.inner.as_ptr();
    let raw_ctx = unsafe { JS_NewContext(raw_runtime) };
    Self(NonNull::new(raw_ctx).expect("non-null context"))
  }

  pub fn from_raw(raw: *mut JSContext) -> Self {
    Self(NonNull::new(raw).expect("non-null context"))
  }

  pub fn eval(
    &self,
    source: Source,
    options: EvalOptions,
  ) -> Result<Owned<Value>, EvalError> {
    let code = source.to_raw_code();
    let flags = source.to_raw_type() | options.flags.to_raw_flag();
    let name = options.name;
    let value = unsafe {
      JS_Eval(
        self.0.as_ptr(),
        code.as_ptr() as *const c_char,
        code.len(),
        name.as_ptr() as *const c_char,
        flags as c_int,
      )
    };
    self.to_owned_value_or_error(Value(value))
  }

  fn to_owned_value_or_error(
    &self,
    value: Value,
  ) -> Result<Owned<Value>, EvalError> {
    if value.is_exception() {
      unimplemented!()
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
    let inner = NonNull::new(unsafe { JS_DupContext(self.0.as_ptr()) })
      .expect("non-null context");
    Context(inner)
  }
}

pub struct EvalOptions<'a> {
  name: &'a str,
  flags: Flag,
}

impl<'a> Default for EvalOptions<'a> {
  fn default() -> Self {
    Self {
      name: "init",
      flags: Flag::None,
    }
  }
}

#[derive(Debug)]
pub enum EvalError {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::runtime::{Runtime, RuntimeOptions};
  use crate::value::Int32;

  #[test]
  fn test_eval() {
    let runtime = Runtime::new(RuntimeOptions::default());
    let context = Context::new(&runtime);
    let source = Source::Global(String::from("40 + 2"));
    let value = context.eval(source, EvalOptions::default()).expect("42");
    let expected = Value::from(Int32::new(&context, 42));
    assert!(value == expected);
    assert!(value == Owned::new(context, expected));
  }
}
