use crate::context::{Context, EvalFlag, EvalType};
use crate::handle::Owned;
use crate::runtime::Runtime;
use crate::value::Value;
use anyhow::Error;

pub struct Worker {
  pub context: Context,
  source: Source,
}

impl Worker {
  pub fn new(runtime: &Runtime, source: Source) -> Self {
    Self {
      source,
      context: Context::new(runtime),
    }
  }

  pub fn run(&self) -> Result<Owned<Value>, Error> {
    let (text, path) = self.source.clone().load()?;
    self
      .context
      .evaluate(text, path, EvalType::Script(EvalFlag::None))
  }

  pub fn fetch(&self) -> Result<Owned<Value>, Error> {
    unimplemented!()
  }
}

#[derive(Clone)]
pub enum Source {
  File(String),
  Text(String),
}

impl Source {
  fn load(self) -> Result<(String, String), Error> {
    let text = match self {
      Self::Text(text) => (text, String::from("init")),
      Self::File(path) => (std::fs::read_to_string(&path)?, path),
    };
    Ok(text)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::value::Int32;

  #[test]
  fn worker() {
    let runtime = Runtime::new(Default::default());
    let source = Source::Text(String::from("40 + 2"));
    let worker = Worker::new(&runtime, source);
    let value = worker.run().expect("42");
    assert!(value == Value::from(Int32::new(&worker.context, 42)));
  }
}
