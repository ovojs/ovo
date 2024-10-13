use crate::{Runtime, Value};
use anyhow::Error;
use ovo_quickjs::*;
use std::{mem::forget, ptr::NonNull, sync::Arc};

#[derive(Clone)]
pub struct Context {
  pub(crate) inner: NonNull<JSContext>,
}

impl Context {
  pub fn new(rt: &Runtime) -> Self {
    let c_context = unsafe { JS_NewContext(rt.inner.as_ptr()) };
    let inner = NonNull::new(c_context).unwrap();
    Self { inner }
  }

  pub fn eval(&self, source: &str) -> Result<Owned<Value>, Error> {
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

pub struct Owned<T>
where
  T: DropFromContext,
{
  data: NonNull<T>,
  context: Arc<Context>,
}

impl<T> Owned<T>
where
  T: DropFromContext,
{
  pub fn new(ctx: &Context, raw: *mut T) -> Self {
    let data = NonNull::new(raw).unwrap();
    let context = Arc::new(ctx.clone());
    Self { data, context }
  }

  #[inline(always)]
  pub fn into_raw(self) -> NonNull<T> {
    let data = self.data;
    forget(self);
    data
  }
}

impl<T> Drop for Owned<T>
where
  T: DropFromContext,
{
  fn drop(&mut self) {
    unsafe { self.data.as_mut().drop_from_context(&self.context) };
  }
}
