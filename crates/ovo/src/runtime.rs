use ovo_quickjs::*;
use std::ptr::NonNull;

pub struct Runtime {
  pub(crate) inner: NonNull<JSRuntime>, 
}

impl Runtime {
  pub fn new() -> Self {
    let inner = match NonNull::new(unsafe { JS_NewRuntime() }) {
      Some(inner) => inner,
      None => panic!("null runtime"),
    };
    Self { inner }
  }
}
