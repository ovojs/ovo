use crate::quickjs::*;
use anyhow::Error;
use std::ptr::NonNull;
use url::Url;

type ModuleSpecifier = Url;

pub trait ModuleLoader {
  fn resolve(
    &self,
    specifier: &str,
    referer: &str,
  ) -> Result<ModuleSpecifier, Error>;

  fn load(&self, specifier: &ModuleSpecifier) -> Result<Module, Error>;
}

pub struct Module(pub(crate) NonNull<JSModuleDef>);
