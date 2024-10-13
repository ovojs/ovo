use anyhow::{anyhow, Error};
use ovo_quickjs::*;
use std::ptr::NonNull;
use url::Url;

type ModuleSpecifier = Url;

pub trait ModuleLoader {
  fn resolve(&self, specifier: &str) -> Result<ModuleSpecifier, Error>;
  fn load(&self, specifier: &ModuleSpecifier) -> Result<Module, Error>;
}

pub struct Module {
  pub(crate) inner: NonNull<JSModuleDef>,
}

pub(crate) struct NoopModuleLoader;

impl ModuleLoader for NoopModuleLoader {
  fn resolve(&self, _: &str) -> Result<ModuleSpecifier, Error> {
    Err(anyhow!("NoopModuleLoader.resolve is called"))
  }

  fn load(&self, _: &ModuleSpecifier) -> Result<Module, Error> {
    Err(anyhow!("NoopModuleLoader.load is called"))
  }
}
