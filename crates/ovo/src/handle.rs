use crate::Context;
use std::borrow::Borrow;
use std::ptr::NonNull;
use std::sync::Arc;

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
    let context = Arc::from(ctx.clone());
    Self { data, context }
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

impl<T> Borrow<T> for Owned<T>
where
  T: DropFromContext,
{
  fn borrow(&self) -> &T {
    unsafe { self.data.as_ref() }
  }
}
