use ovo_quickjs::*;
use std::ptr::NonNull;

pub struct Context(pub(crate) NonNull<JSContext>);

impl Drop for Context {
  fn drop(&mut self) {
    unsafe {
      JS_FreeContext(self.0.as_ptr());
    }
  }
}

impl Clone for Context {
  fn clone(&self) -> Self {
    unsafe { Context(NonNull::new(JS_DupContext(self.0.as_ptr())).unwrap()) }
  }
}
