use crate::context::Context;
use std::borrow::Borrow;

pub trait DropFromContext {
  fn drop_from_context(&mut self, ctx: &Context);
}

pub trait CloneFromContext {
  fn clone_from_context(&self, ctx: &Context) -> Self;
}

pub trait PartialEqFromContext {
  fn eq_from_context(&self, ctx: &Context, other: &Self) -> bool;
}

pub struct Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  ctx: Context,
  data: T,
}

impl<T> Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  pub fn new(ctx: Context, data: T) -> Self {
    Self { ctx, data }
  }
}

impl<T> Drop for Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  fn drop(&mut self) {
    self.data.drop_from_context(&self.ctx);
  }
}

impl<T> Clone for Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  fn clone(&self) -> Self {
    let ctx = self.ctx.clone();
    let data = self.data.clone_from_context(&ctx);
    Self { data, ctx }
  }
}

impl<T> Borrow<T> for Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  fn borrow(&self) -> &T {
    &self.data
  }
}

impl<T> PartialEq<T> for Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  fn eq(&self, other: &T) -> bool {
    self.data.eq_from_context(&self.ctx, other)
  }
}

impl<T> PartialEq for Owned<T>
where
  T: DropFromContext + CloneFromContext + PartialEqFromContext,
{
  fn eq(&self, other: &Self) -> bool {
    self.data.eq_from_context(&self.ctx, &other.data)
  }
}
