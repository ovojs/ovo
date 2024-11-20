use url::{ParseError, Url};

use crate::context::Context;
use crate::error::Error;
use crate::module::{Module, ModuleSpecifier};

pub trait ModuleLoader {
  fn load(
    &self,
    ctx: &Context,
    specifier: ModuleSpecifier,
  ) -> Result<Module, Error>;

  fn resolve(
    &self,
    ctx: &Context,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error>;
}

pub struct ExtModuleLoader {}

impl ExtModuleLoader {
  pub fn new() -> Self {
    Self {}
  }
}

impl ModuleLoader for ExtModuleLoader {
  fn load(
    &self,
    ctx: &Context,
    specifier: ModuleSpecifier,
  ) -> Result<Module, Error> {
    _ = ctx;
    _ = specifier;
    unimplemented!()
  }

  fn resolve(
    &self,
    _ctx: &Context,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error> {
    resolve_import(specifier, referer)
  }
}

/// Resolves module specifier from a referer.
/// Ref: https://github.com/denoland/deno_core/blob/bbc460e7b1a783ff623c15f02d0ad1a26c49f005/core/module_specifier.rs
fn resolve_import(
  specifier: &str,
  referer: &str,
) -> Result<ModuleSpecifier, Error> {
  let url = match Url::parse(specifier) {
    Ok(url) => url,
    Err(ParseError::RelativeUrlWithoutBase) => {
      if !(specifier.starts_with('/')
        || specifier.starts_with("./")
        || specifier.starts_with("../"))
      {
        let maybe_referer = if referer.is_empty() {
          None
        } else {
          Some(referer.to_string())
        };
        return Err(Error::ImportPrefixMissing(
          specifier.to_string(),
          maybe_referer,
        ));
      }
      let referer = Url::parse(referer).map_err(Error::InvalidBaseUrl)?;
      referer.join(specifier).map_err(Error::InvalidUrl)?
    }
    Err(err) => return Err(Error::InvalidUrl(err)),
  };
  Ok(url)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resolve_import() {
    let cases = vec![
      (
        "./yyy/xxx.ts",
        "file:///aaa/bbb/ccc/ddd.ts",
        "file:///aaa/bbb/ccc/yyy/xxx.ts",
      ),
      (
        "../yyy/xxx.ts",
        "file:///aaa/bbb/ccc/ddd.ts",
        "file:///aaa/bbb/yyy/xxx.ts",
      ),
      (
        "../../yyy/xxx.ts",
        "file:///aaa/bbb/ccc/ddd.ts",
        "file:///aaa/yyy/xxx.ts",
      ),
      (
        "/yyy/xxx.ts",
        "file:///aaa/bbb/ccc/ddd.ts",
        "file:///yyy/xxx.ts",
      ),
      (
        "file:///yyy/xxx.ts",
        "file:///aaa/bbb/ccc/ddd.ts",
        "file:///yyy/xxx.ts",
      ),
      (
        "ovo:console",
        "file:///aaa/bbb/ccc/ddd/eeeeeee.ts",
        "ovo:console",
      ),
    ];
    for (specifier, referer, expected_url) in cases {
      let url = resolve_import(specifier, referer).unwrap().to_string();
      assert_eq!(url, expected_url);
    }
  }
}
