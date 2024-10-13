use anyhow::{Error, Ok};
use ovo_quickjs::{JS_Eval, JS_EVAL_TYPE_GLOBAL, JS_EVAL_TYPE_MODULE};

use crate::{context::Owned, Context, Value};

pub struct Script {
  name: String,
  source: String,
}

impl Script {
  pub fn new(source: &str) -> Self {
    Self {
      name: String::from("anon"),
      source: String::from(source),
    }
  }

  pub fn eval(&self, ctx: &Context) -> Result<Owned<Value>, Error> {
    let c_value = unsafe {
      JS_Eval(
        ctx.inner.as_ptr(),
        self.source.as_ptr() as *const i8,
        self.source.len(),
        self.name.as_ptr() as *const i8,
        JS_EVAL_TYPE_MODULE as i32,
      )
    };
    Ok(Owned::new(ctx, &mut Value(c_value) as *mut Value))
  }
}
