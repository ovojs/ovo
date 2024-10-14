mod context;
mod data;
mod handle;
mod module;
mod primitive;
mod runtime;
mod script;
mod value;

pub use context::Context;
pub use data::*;
pub use handle::Owned;
pub use module::Module;
pub use runtime::Runtime;
pub use script::Script;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let _runtime = &Runtime::new(Default::default());
    let _context = &Context::new(_runtime);
    // let script = Script::new("import { A } from './A.js'; 40 + 2");
    // let value = script.eval(context);
  }
}
