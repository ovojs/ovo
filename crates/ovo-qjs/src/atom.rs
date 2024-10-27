use crate::context::Context;
use crate::ffi::*;
use crate::handle::DropFromContext;

pub struct Atom(pub(crate) JSAtom);

impl Atom {
  pub fn new(ctx: &Context, sym: &str) -> Self {
    unsafe { Self(JS_NewAtom(ctx.0.as_ptr(), sym.as_ptr() as *const i8)) }
  }
}

impl DropFromContext for Atom {
  fn drop_from_context(&mut self, ctx: &Context) {
    unsafe { JS_FreeAtom(ctx.0.as_ptr(), self.0) }
  }
}
