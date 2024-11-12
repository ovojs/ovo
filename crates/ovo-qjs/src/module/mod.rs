pub mod loader;

use crate::ffi::*;
use std::ptr::NonNull;
use url::Url;

pub type ModuleSpecifier = Url;

pub struct Module(pub(crate) NonNull<JSModuleDef>);

impl Module {}
