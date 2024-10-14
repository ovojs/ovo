use crate::{Context, Owned, String, Value};
use anyhow::Error;

pub struct ScriptOrigin {
  name: String,
  is_module: bool,
}

pub struct Script {
  func: Owned<Value>,
}

impl Script {
  pub fn compile(
    ctx: &Context,
    input: &str,
    origin: Option<ScriptOrigin>,
  ) -> Self {
    unimplemented!()
  }

  pub fn run(ctx: &Context) -> Result<Owned<Value>, Error> {
    unimplemented!()
  }
}
