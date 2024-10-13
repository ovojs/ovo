use crate::{Runtime, Value};
use anyhow::Error;
use ovo_quickjs::*;
use std::{ptr::NonNull, sync::Arc};

pub struct Context {
  pub(crate) inner: NonNull<JSContext>,
}

impl Context {
  pub fn new(rt: &Runtime) -> Self {
    let c_context = unsafe { JS_NewContext(rt.inner.as_ptr()) };
    let inner = NonNull::new(c_context).unwrap();
    Self { inner }
  }

  pub fn eval(&self) -> Result<Owned<Value>, Error> {
    unimplemented!()
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { JS_FreeContext(self.inner.as_ptr()) }
  }
}

pub trait DropFromContext {
  fn drop_from_context(&mut self, ctx: &Context);
}

pub struct Owned<T: DropFromContext> {
  data: NonNull<T>,
  context: Arc<Context>,
}

impl<T> Drop for Owned<T>
where
  T: DropFromContext,
{
  fn drop(&mut self) {
    unsafe { self.data.as_mut().drop_from_context(&self.context) };
  }
}
