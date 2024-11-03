use crate::extension::Op;

macro_rules! property_fn {
  ($op:expr) => {
    crate::ffi::JSCFunctionListEntry {
      name: $op.name.as_ptr() as *const i8,
      prop_flags: crate::ffi::JS_PROP_CONFIGURABLE as u8,
      def_type: crate::ffi::JS_DEF_CFUNC as u8,
      magic: $op.magic,
      u: crate::ffi::JSCFunctionListEntry__bindgen_ty_1 {
        func: crate::ffi::JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1 {
          length: $op.argc,
          cproto: crate::ffi::JSCFunctionEnum_JS_CFUNC_generic as u8,
          cfunc: crate::ffi::JSCFunctionType { generic: $op.r#fn },
        },
      },
    }
  };
}

macro_rules! property_i32 {
  ($name:expr, $value:expr) => {
    crate::ffi::JSCFunctionListEntry {
      name: $name.as_ptr() as *const i8,
      prop_flags: crate::ffi::JS_PROP_CONFIGURABLE as u8,
      def_type: crate::ffi::JS_DEF_PROP_INT32 as u8,
      magic: 0,
      u: crate::ffi::JSCFunctionListEntry__bindgen_ty_1 { i32_: $value },
    }
  };
}

macro_rules! property_i64 {
  ($name:expr, $value:expr) => {
    crate::ffi::JSCFunctionListEntry {
      name: $name.as_ptr() as *const i8,
      prop_flags: crate::ffi::JS_PROP_CONFIGURABLE as u8,
      def_type: crate::ffi::JS_DEF_PROP_INT32 as u8,
      magic: 0,
      u: crate::ffi::JSCFunctionListEntry__bindgen_ty_1 { i64_: $value },
    }
  };
}

macro_rules! property_f64 {
  ($name:expr, $value:expr) => {
    crate::ffi::JSCFunctionListEntry {
      name: $name.as_ptr() as *const i8,
      prop_flags: crate::ffi::JS_PROP_CONFIGURABLE as u8,
      def_type: crate::ffi::JS_DEF_PROP_INT32 as u8,
      magic: 0,
      u: crate::ffi::JSCFunctionListEntry__bindgen_ty_1 { f64_: $value },
    }
  };
}

#[derive(Clone, Copy)]
pub enum Property {
  Function(Op),
  Int32(&'static str, i32),
  Int64(&'static str, i64),
  Float64(&'static str, f64),
}

impl Property {
  pub fn to_js_cfunction_list_entry(self) -> crate::ffi::JSCFunctionListEntry {
    match self {
      Property::Function(op) => property_fn!(op),
      Property::Int32(name, value) => property_i32!(name, value),
      Property::Int64(name, value) => property_i64!(name, value),
      Property::Float64(name, value) => property_f64!(name, value),
    }
  }
}
