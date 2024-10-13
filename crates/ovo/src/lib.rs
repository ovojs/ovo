mod context;
mod module;
mod runtime;
mod script;
mod value;

pub use context::Context;
pub use module::Module;
pub use runtime::Runtime;
pub use script::Script;
pub use value::Value;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let runtime = &Runtime::new(Default::default());
    let context = &Context::new(runtime);
    let script = Script::new("import { A } from './A.js'; 40 + 2");
    let value = script.eval(context);
    match value {
      Ok(value) => unsafe { assert_eq!(value.into_raw().as_mut().to_i32(context), Some(42)) },
      Err(_) => {}
    }
  }
}
