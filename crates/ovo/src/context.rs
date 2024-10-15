use crate::{Owned, Runtime, String, Value};
use anyhow::Error;
use ovo_quickjs::*;
use std::ptr::NonNull;

pub struct Context(pub(crate) NonNull<JSContext>);

impl Context {
  pub fn new(rt: &Runtime) -> Self {
    let rt_raw = rt.inner.as_ptr();
    let inner = match NonNull::new(unsafe { JS_NewContext(rt_raw) }) {
      Some(inner) => inner,
      None => panic!("null context"),
    };
    Self(inner)
  }

  pub fn eval(&self, source: String) -> Result<Owned<Value>, Error> {
    _ = source;
    unimplemented!()
  }
}

impl Drop for Context {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe {
      JS_FreeContext(self.0.as_ptr());
    }
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
