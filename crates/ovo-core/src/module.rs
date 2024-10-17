use crate::{context::Context, quickjs::*};
use anyhow::Error;
use std::ptr::NonNull;
use url::Url;

pub type ModuleSpecifier = Url;

pub trait ModuleLoader {
  fn resolve(
    &self,
    ctx: &Context,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error>;

  fn load(
    &self,
    ctx: &Context,
    specifier: ModuleSpecifier,
  ) -> Result<Module, Error>;
}

pub struct Module(pub(crate) NonNull<JSModuleDef>);

impl Module {}

pub struct ExtModuleLoader;

impl ExtModuleLoader {
  pub fn new() -> Self {
    Self {}
  }
}

impl ModuleLoader for ExtModuleLoader {
  fn resolve(
    &self,
    _ctx: &Context,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error> {
    resolve_imports(specifier, referer)
  }

  fn load(
    &self,
    ctx: &Context,
    specifier: ModuleSpecifier,
  ) -> Result<Module, Error> {
    unimplemented!()
  }
}

pub fn resolve_imports(
  specifier: &str,
  referer: &str,
) -> Result<ModuleSpecifier, Error> {
  unimplemented!()
}
