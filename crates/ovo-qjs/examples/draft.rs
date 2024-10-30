#[ovo]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[allow(non_camel_case_types)]
struct op_add;

impl ovo_qjs::ext::Op for op_add {
  const NAME: &'static str = "add";
  const DECL: ovo_qjs::ext::OpDecl =
    ovo_qjs::ext::OpDecl::new(2, Some(Self::js_call));
}

impl op_add {
  #[inline(always)]
  fn call(a: i32, b: i32) -> i32 {
    a + b
  }

  unsafe extern "C" fn js_call(
    ctx: *mut ovo_qjs::ffi::JSContext,
    this_val: ovo_qjs::ffi::JSValue,
    argc: std::ffi::c_int,
    argv: *mut ovo_qjs::ffi::JSValue,
  ) -> ovo_qjs::ffi::JSValue {
    let scope = ovo_qjs::ext::CallScope::new(ctx, this_val, argc, argv);
    let Some(arg0) = scope.get(0).try_to_i32(scope.context()) else {
      return scope.throw_type_error("expected i32");
    };
    let Some(arg1) = scope.get(1).try_to_i32(scope.context()) else {
      return scope.throw_type_error("expected i32");
    };
    let result = Self::call(arg0, arg1);
    ovo_qjs::Value::from(ovo_qjs::Int32::new(scope.context(), result)).into()
  }
}

fn main() {}
