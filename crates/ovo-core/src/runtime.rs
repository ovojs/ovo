use crate::context::Context;
use crate::module::{ExtModuleLoader, ModuleLoader, ModuleSpecifier};
use crate::quickjs::*;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::transmute;
use std::ptr::NonNull;
use std::rc::Rc;

pub struct RuntimeOptions {
  pub loader: Rc<dyn ModuleLoader>,
}

impl Default for RuntimeOptions {
  fn default() -> Self {
    Self {
      loader: Rc::new(ExtModuleLoader::new()),
    }
  }
}

pub struct Runtime {
  pub(crate) inner: NonNull<JSRuntime>,
  pub(crate) loader: Rc<dyn ModuleLoader>,
}

impl Runtime {
  pub fn new(options: RuntimeOptions) -> Self {
    let raw = unsafe { JS_NewRuntime() };
    let inner = NonNull::new(raw).expect("non-null runtime");
    let runtime = Self {
      inner,
      loader: options.loader,
    };
    runtime.init_module_loader();
    runtime
  }

  fn init_module_loader(&self) {
    unsafe {
      JS_SetModuleLoaderFunc(
        self.inner.as_ptr(),
        Some(module_normalizer_unsafe),
        Some(module_loader_unsafe),
        Box::into_raw(Box::new(self)) as *mut c_void,
      );
    }
  }
}

#[no_mangle]
unsafe extern "C" fn module_normalizer_unsafe(
  ctx: *mut JSContext,
  module_base_name: *const c_char,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut c_char {
  let context = Context::from_raw(ctx);
  let opaque = NonNull::new(opaque).expect("non-null opaque");
  let runtime = transmute::<*mut c_void, *mut Runtime>(opaque.as_ptr());
  let module_name = CStr::from_ptr(module_name)
    .to_str()
    .expect("utf8 module name");
  let module_base_name = CStr::from_ptr(module_base_name)
    .to_str()
    .expect("utf8 module base name");
  let specifier = (*runtime)
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
  let runtime = transmute::<*mut c_void, *mut Runtime>(opaque.as_ptr());
  // Re-take the ownership of module_name returned by module_normalizer
  let binding = CString::from_raw(module_name as *mut c_char);
  let specifier = binding.as_c_str().to_str().expect("specifier");
  let specifier = ModuleSpecifier::parse(specifier).expect("parse specifier");
  (*runtime)
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
