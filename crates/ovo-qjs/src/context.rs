use crate::atom::Atom;
use crate::error::Error;
use crate::ffi::*;
use crate::handle::Owned;
use crate::module::Module;
use crate::runtime::Runtime;
use crate::source::{Flag, Source};
use crate::value::{Object, Value};
use std::ffi::{c_char, c_int, c_void};
use std::ptr::NonNull;

pub struct Context(pub NonNull<JSContext>);

impl Context {
  pub fn new(runtime: &Runtime) -> Self {
    let raw_runtime = runtime.inner.as_ptr();
    let raw_ctx = unsafe { JS_NewContext(raw_runtime) };
    Self(NonNull::new(raw_ctx).expect("non-null context"))
  }

  pub fn from_raw(raw: *mut JSContext) -> Self {
    Self(NonNull::new(raw).expect("non-null context"))
  }

  pub fn set_class_proto(&self, class_id: u32, obj: Value) {
    unsafe { JS_SetClassProto(self.0.as_ptr(), class_id, obj.into()) }
  }

  pub fn set_constructor(&self, func_obj: Value, proto: Value) {
    unsafe { JS_SetConstructor(self.0.as_ptr(), func_obj.into(), proto.into()) }
  }

  pub fn set_constructor_bit(
    &self,
    func_obj: Value,
    value: i32,
  ) -> Result<bool, Error> {
    self.to_bool_or_error(unsafe {
      JS_SetConstructorBit(self.0.as_ptr(), func_obj.into(), value as c_int)
    })
  }

  pub fn set_opaque<T>(&self, opaque: NonNull<T>) {
    unsafe {
      JS_SetContextOpaque(self.0.as_ptr(), opaque.as_ptr() as *mut c_void)
    }
  }

  pub fn set_is_html_dda(&self, obj: Value) {
    unsafe { JS_SetIsHTMLDDA(self.0.as_ptr(), obj.into()) }
  }

  pub fn set_module_export(
    &self,
    module: Module,
    name: &str,
    value: Value,
  ) -> Result<(), Error> {
    self.to_void_or_error(unsafe {
      JS_SetModuleExport(
        self.0.as_ptr(),
        module.0.as_ptr(),
        name.as_ptr() as *const i8,
        value.into(),
      )
    })
  }

  pub fn set_property(
    &self,
    this_obj: Object,
    name: Atom,
    prop: Value,
  ) -> Result<bool, Error> {
    self.to_bool_or_error(unsafe {
      JS_SetProperty(self.0.as_ptr(), this_obj.0, name.0, prop.into())
    })
  }

  pub fn set_prototype(&self, obj: Value, proto: Value) -> Result<bool, Error> {
    self.to_bool_or_error(unsafe {
      JS_SetPrototype(self.0.as_ptr(), obj.into(), proto.into())
    })
  }

  pub fn get_global_object(&self) -> Value {
    Value(unsafe { JS_GetGlobalObject(self.0.as_ptr()) })
  }

  pub fn add_module_export(
    &self,
    module: Module,
    name: &str,
  ) -> Result<(), Error> {
    self.to_void_or_error(unsafe {
      JS_AddModuleExport(
        self.0.as_ptr(),
        module.0.as_ptr(),
        name.as_ptr() as *const i8,
      )
    })
  }

  pub fn is_error(&self, value: Value) -> bool {
    unsafe { JS_IsError(self.0.as_ptr(), value.into()) != 0 }
  }

  pub fn eval(
    &self,
    source: Source,
    options: EvalOptions,
  ) -> Result<Owned<Value>, Error> {
    let code = source.to_raw_code();
    let flags = source.to_raw_type() | options.flags.to_raw_flag();
    let name = options.name;
    self.to_owned_value_or_error(Value(unsafe {
      JS_Eval(
        self.0.as_ptr(),
        code.as_ptr() as *const c_char,
        code.len(),
        name.as_ptr() as *const c_char,
        flags as c_int,
      )
    }))
  }

  #[inline(always)]
  fn to_owned_value_or_error(
    &self,
    value: Value,
  ) -> Result<Owned<Value>, Error> {
    if value.is_exception() {
      Err(Error::Eval)
    } else {
      Ok(Owned::new(self.clone(), value))
    }
  }

  #[inline(always)]
  fn to_bool_or_error(&self, rev: i32) -> Result<bool, Error> {
    if rev != -1 {
      Ok(rev != 0)
    } else {
      todo!("JS_GetException");
    }
  }

  #[inline(always)]
  fn to_void_or_error(&self, rev: i32) -> Result<(), Error> {
    if rev != -1 {
      Ok(())
    } else {
      todo!("JS_GetException");
    }
  }
}

impl Drop for Context {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { JS_FreeContext(self.0.as_ptr()) }
  }
}

impl Clone for Context {
  fn clone(&self) -> Self {
    let inner = NonNull::new(unsafe { JS_DupContext(self.0.as_ptr()) })
      .expect("non-null context");
    Context(inner)
  }
}

pub struct EvalOptions<'a> {
  name: &'a str,
  flags: Flag,
}

impl<'a> Default for EvalOptions<'a> {
  fn default() -> Self {
    Self {
      name: "init",
      flags: Flag::None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::runtime::{Runtime, RuntimeOptions};
  use crate::value::Int32;

  #[test]
  fn test_eval() {
    let runtime = Runtime::new(RuntimeOptions::default());
    let context = Context::new(&runtime);
    let source = Source::Global(String::from("40 + 2"));
    let value = context.eval(source, EvalOptions::default()).expect("42");
    let expected = Value::from(Int32::new(&context, 42));
    assert!(value == expected);
    assert!(value == Owned::new(context, expected));
  }
}
