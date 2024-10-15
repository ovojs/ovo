use crate::Context;
use std::borrow::Borrow;
use std::sync::Arc;

pub trait DropFromContext {
  fn drop_from_context(&mut self, ctx: &Context);
}

pub trait CloneFromContext {
  fn clone_from_context(&self, ctx: &Context) -> Self;
}

pub struct Owned<T>
where
  T: DropFromContext + CloneFromContext,
{
  ctx: Arc<Context>,
  data: T,
}

impl<T> Owned<T>
where
  T: DropFromContext + CloneFromContext,
{
  pub fn new(ctx: Arc<Context>, data: T) -> Self {
    Self { ctx, data }
  }
}

impl<T> Drop for Owned<T>
where
  T: DropFromContext + CloneFromContext,
{
  fn drop(&mut self) {
    self.data.drop_from_context(&self.ctx);
  }
}

impl<T> Clone for Owned<T>
where
  T: DropFromContext + CloneFromContext,
{
  fn clone(&self) -> Self {
    let ctx = self.ctx.clone();
    let data = self.data.clone_from_context(&ctx);
    Self { data, ctx }
  }
}

impl<T> Borrow<T> for Owned<T>
where
  T: DropFromContext + CloneFromContext,
{
  fn borrow(&self) -> &T {
    self.data.borrow()
  }
}
