use crate::context::Context;
use crate::module::{ModuleLoader, ModuleSpecifier};
use crate::quickjs::*;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::transmute;
use std::ptr::NonNull;

pub struct Runtime(pub(crate) NonNull<JSRuntime>);

impl Runtime {
  pub fn new() -> Self {
    let raw = unsafe { JS_NewRuntime() };
    Self(NonNull::new(raw).expect("non-null runtime"))
  }

  pub fn set_module_loader(&self, loader: impl ModuleLoader) {
    unsafe {
      JS_SetModuleLoaderFunc(
        self.0.as_ptr(),
        Some(module_normalizer),
        Some(module_loader),
        Box::into_raw(Box::new(loader)) as *mut c_void,
      );
    }
  }
}

#[no_mangle]
unsafe extern "C" fn module_normalizer(
  ctx: *mut JSContext,
  module_base_name: *const c_char,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut c_char {
  let context = Context::from_raw(ctx);
  let opaque = NonNull::new(opaque).expect("non-null opaque");
  // FIXME
  let loader = transmute::<*mut c_void, *mut dyn ModuleLoader>(opaque.as_ptr());
  let module_name = CStr::from_ptr(module_name)
    .to_str()
    .expect("utf8 module name");
  let module_base_name = CStr::from_ptr(module_base_name)
    .to_str()
    .expect("utf8 module base name");
  let specifier = (*loader)
    .resolve(&context, module_name, module_base_name)
    .expect("module resolvation");
  CString::new(specifier.as_str())
    .expect("specifier")
    .into_raw()
}

#[no_mangle]
unsafe extern "C" fn module_loader(
  ctx: *mut JSContext,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut JSModuleDef {
  let context = Context::from_raw(ctx);
  let opaque = NonNull::new(opaque).expect("non-null opaque");
  // FIXME
  let loader = transmute::<*mut c_void, *mut dyn ModuleLoader>(opaque.as_ptr());
  // Re-take the ownership of module_name returned by module_normalizer
  let specifier = CString::from_raw(module_name as *mut c_char);
  let specifier = specifier.as_c_str().to_str().expect("specifier");
  let specifier = ModuleSpecifier::parse(specifier).expect("parse specifier");
  (*loader)
    .load(&context, specifier)
    .expect("module loading")
    .0
    .as_ptr()
}

impl Drop for Runtime {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { JS_FreeRuntime(self.0.as_ptr()) }
  }
}
