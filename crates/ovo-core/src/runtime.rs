use crate::context::Context;
use crate::module::{
  ExtModuleLoader, ModuleLoader, ModuleSpecifier, ModuleSpecifierMap,
};
use crate::quickjs::*;
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::NonNull;
use std::rc::Rc;

#[derive(Default)]
pub struct RuntimeOptions {
  pub loader: Option<Rc<dyn ModuleLoader>>,
}

pub struct Runtime {
  pub(crate) inner: NonNull<JSRuntime>,
  pub(crate) loader: Rc<dyn ModuleLoader>,
}

impl Runtime {
  pub fn new(options: RuntimeOptions) -> Self {
    let raw = unsafe { JS_NewRuntime() };
    let inner = NonNull::new(raw).expect("non-null runtime");
    let loader = options.loader.unwrap_or_else(|| {
      Rc::new(ExtModuleLoader::new(ModuleSpecifierMap::new()))
    });
    Self { inner, loader }
  }

  pub fn init_module_loader(&self) {
    unsafe {
      JS_SetModuleLoaderFunc(
        self.inner.as_ptr(),
        Some(module_resolver_unsafe),
        Some(module_loader_unsafe),
        self as *const Runtime as *const () as *mut c_void,
      );
    }
  }
}

#[no_mangle]
unsafe extern "C" fn module_resolver_unsafe(
  ctx: *mut JSContext,
  module_base_name: *const c_char,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut c_char {
  let context = Context::from_raw(ctx);
  let opaque = NonNull::new(opaque).expect("non-null opaque");
  let runtime = opaque.as_ptr() as *const Runtime;
  let module_name = CStr::from_ptr(module_name)
    .to_str()
    .expect("utf8 module name");
  let module_base_name = CStr::from_ptr(module_base_name)
    .to_str()
    .expect("utf8 module base name");
  let specifier = (&*runtime)
    .loader
    .resolve(&context, module_name, module_base_name)
    .expect("resolve module");
  // here CString gives up the ownership of specifier by calling
  // into_raw, so module_loader_unsafe must re-take the ownership
  CString::new(specifier.as_str())
    .expect("specifier")
    .into_raw()
}

#[no_mangle]
unsafe extern "C" fn module_loader_unsafe(
  ctx: *mut JSContext,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut JSModuleDef {
  let context = Context::from_raw(ctx);
  let opaque = NonNull::new(opaque).expect("non-null opaque");
  let runtime = opaque.as_ptr() as *mut Runtime;
  // Re-take the ownership of module_name returned by module_resolver_unsafe
  let binding = CString::from_raw(module_name as *mut c_char);
  let specifier = binding.as_c_str().to_str().expect("specifier");
  let specifier = ModuleSpecifier::parse(specifier).expect("parse specifier");
  (&*runtime)
    .loader
    .load(&context, specifier)
    .expect("load module")
    .0
    .as_ptr()
}

impl Drop for Runtime {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { JS_FreeRuntime(self.inner.as_ptr()) }
  }
}
