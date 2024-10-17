use crate::handle::Owned;
use crate::module::ModuleSpecifier;
use crate::quickjs::*;
use crate::runtime::Runtime;
use crate::value::{String, Value};
use anyhow::Error;
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

  pub fn eval(&self, source: String) -> Result<Owned<Value>, Error> {
    _ = source;
    unimplemented!()
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
