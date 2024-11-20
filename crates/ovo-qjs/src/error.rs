use url::ParseError;

#[derive(Debug)]
pub enum Error {
  Evaluate,
  Internal(&'static str),
  InvalidUrl(ParseError),
  InvalidBaseUrl(ParseError),
  ImportPrefixMissing(String, Option<String>),
}
