use ovo_qjs::{ext, Context, EvalOptions, Runtime, RuntimeOptions, Source};

// #[ovo]
// fn add(a: i32, b: i32) -> i32 {
//   a + b
// }

// ext!(test, ops = [add]);

#[allow(non_camel_case_types)]
struct add;

impl add {
  pub fn new() -> ovo_qjs::Op {
    ovo_qjs::Op {
      name: "add",
      r#fn: Some(Self::js_call),
    }
  }

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
    let scope = ovo_qjs::CallScope::new(ctx, this_val, argc, argv);
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

#[allow(non_camel_case_types)]
struct test;

impl test {
  pub fn new() -> ovo_qjs::Ext {
    ovo_qjs::Ext {
      name: "test",
      ops: std::borrow::Cow::Owned(vec![add::new()]),
    }
  }
}

fn main() {
  let test_ext = test::new();
  let runtime = Runtime::new(RuntimeOptions {
    extensions: vec![test_ext],
    ..Default::default()
  });
  let context = Context::new(&runtime);
  let source = Source::Module(
    r#"
import { add } from "ovo:test";

console.log(add(40, 2))    
    "#
    .to_string(),
  );
  context.eval(source, EvalOptions::default()).expect("eval");
}
