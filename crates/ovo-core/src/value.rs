use crate::context::Context;
use crate::data::Value;
use crate::handle::{CloneFromContext, DropFromContext};
use crate::quickjs::*;

impl Value {}

impl DropFromContext for Value {
  fn drop_from_context(&mut self, ctx: &Context) {
    unsafe { JS_FreeValue(ctx.0.as_ptr(), self.0) };
  }
}

impl CloneFromContext for Value {
  fn clone_from_context(&self, ctx: &Context) -> Self {
    unsafe { Self(JS_DupValue(ctx.0.as_ptr(), self.0)) }
  }
}
