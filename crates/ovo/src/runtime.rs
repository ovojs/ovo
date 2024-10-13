use crate::module::{ModuleLoader, NoopModuleLoader};
use ovo_quickjs::*;
use std::ffi::{c_char, c_void};
use std::ptr::NonNull;
use std::rc::Rc;

pub struct Runtime {
  pub(crate) inner: NonNull<JSRuntime>,
}

impl Runtime {
  pub fn new(options: RuntimeOptions) -> Self {
    let c_runtime = unsafe { JS_NewRuntime() };
    let inner = NonNull::new(c_runtime).unwrap();
    Runtime::init(c_runtime, options);
    Self { inner }
  }

  fn init(inner: *mut JSRuntime, options: RuntimeOptions) {
    let loader = options
      .module_loader
      .unwrap_or(Rc::new(NoopModuleLoader {}));
    unsafe {
      JS_SetModuleLoaderFunc(
        inner,
        Some(js_module_normalizer),
        Some(js_module_loader),
        Box::into_raw(Box::new(loader.as_ref())) as *mut c_void,
      );
    }
  }
}

impl Drop for Runtime {
  fn drop(&mut self) {
    unsafe { JS_FreeRuntime(self.inner.as_ptr()) }
  }
}

#[derive(Default)]
pub struct RuntimeOptions {
  pub module_loader: Option<Rc<dyn ModuleLoader>>,
}

#[no_mangle]
extern "C" fn js_module_normalizer(
  ctx: *mut JSContext,
  module_base_name: *const c_char,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut c_char {
  unimplemented!()
}

#[no_mangle]
extern "C" fn js_module_loader(
  ctx: *mut JSContext,
  module_name: *const c_char,
  opaque: *mut c_void,
) -> *mut JSModuleDef {
  unimplemented!()
}

#[cfg(test)]
mod tests {

  #[test]
  fn basic() {}
}
