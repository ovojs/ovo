use crate::context::Context;
use crate::handle::Owned;
use crate::runtime::{Runtime, RuntimeOptions};
use crate::value::Value;
use anyhow::Error;
use std::sync::Arc;

pub struct Worker {
  runtime: Runtime,
  context: Arc<Context>,
}

impl Worker {
  pub fn new(options: WorkerOptions) -> Self {
    let runtime = Runtime::new(RuntimeOptions {
      ..RuntimeOptions::default()
    });
    let context = Context::new(&runtime);
    Self {
      runtime,
      context: Arc::new(context),
    }
  }

  pub fn fetch(&self) -> Result<Owned<Value>, Error> {
    unimplemented!()
  }
}

pub struct WorkerOptions {}

impl Default for WorkerOptions {
  fn default() -> Self {
    Self {}
  }
}
