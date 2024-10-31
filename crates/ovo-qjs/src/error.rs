#[derive(Debug)]
pub enum Error {
  Evaluate,
  Internal(&'static str),
}
