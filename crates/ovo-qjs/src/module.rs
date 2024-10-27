use crate::context::Context;
use crate::ffi::*;
use anyhow::Error;
use std::collections::HashMap;
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

pub type ModuleSpecifierMap = HashMap<String, ModuleSpecifier>;

pub struct ExtModuleLoader {
  specifier_map: ModuleSpecifierMap,
}

impl ExtModuleLoader {
  pub fn new(specifier_map: ModuleSpecifierMap) -> Self {
    Self { specifier_map }
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
    _ = ctx;
    _ = specifier;
    unimplemented!()
  }
}

pub fn resolve_imports(
  specifier: &str,
  referer: &str,
) -> Result<ModuleSpecifier, Error> {
  _ = specifier;
  _ = referer;
  unimplemented!()
}
