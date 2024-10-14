use crate::{Owned, Runtime, Value};
use anyhow::{anyhow, Error, Ok};
use ovo_quickjs::{
  JSContext, JS_Eval, JS_FreeContext, JS_IsException, JS_NewContext,
  JS_StrictEq, JS_EVAL_TYPE_MODULE,
};
use std::{
  ffi::{c_char, c_int},
  ptr::NonNull,
};

#[derive(Clone)]
pub struct Context {
  pub(crate) inner: NonNull<JSContext>,
}

impl Context {
  pub fn new(rt: &Runtime) -> Self {
    let raw = unsafe { JS_NewContext(rt.inner.as_ptr()) };
    let inner = NonNull::new(raw).unwrap();
    Self { inner }
  }

  pub fn eval(&self, input: &str) -> Result<Owned<Value>, Error> {
    let value = unsafe {
      let value = JS_Eval(
        self.inner.as_ptr(),
        input.as_ptr() as *const c_char,
        input.len(),
        "<init>".as_ptr() as *const c_char,
        JS_EVAL_TYPE_MODULE as c_int,
      );
      if JS_IsException(value) == 0 {
        return Err(anyhow!("todo: exception"));
      } else {
        value
      }
    };
    Ok(Owned::new(self, Box::into_raw(Box::new(Value(value)))))
  }

  pub fn struct_eq(&self, v1: &Value, v2: &Value) -> bool {
    unsafe { JS_StrictEq(self.inner.as_ptr(), v1.0, v2.0) != 0 }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { JS_FreeContext(self.inner.as_ptr()) }
  }
}

#[cfg(test)]
mod tests {
  use std::borrow::Borrow;

  use crate::{Context, Int32, Runtime, Value};

  #[test]
  fn basic() {
    let rt = &Runtime::new(Default::default());
    let ctx = &Context::new(rt);
    let value = ctx.eval("import * as from 'ovo'; 40 + 2");
    assert!(value.is_ok());
    let expected: Value = Int32::new(ctx, 42).into();
    assert!(ctx.struct_eq(value.unwrap().borrow(), &expected));
  }
}
