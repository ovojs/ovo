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
    ctx: &Context,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error> {
    _ = ctx;
    _ = specifier;
    _ = referer;
    unimplemented!()
  }
}
