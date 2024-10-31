#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    unsafe {
      let rt = JS_NewRuntime();
      let ctx = JS_NewContext(rt);

      let code = String::from("40 + 2");
      let name = String::from("init");

      let val = JS_Eval(
        ctx,
        code.as_ptr() as *const i8,
        code.len(),
        name.as_ptr() as *const i8,
        JS_EVAL_TYPE_GLOBAL as i32,
      );

      let eq = JS_IsStrictEqual(ctx, val, JS_NewInt32(ctx, 42));
      assert_eq!(eq, true as i32);

      JS_FreeValue(ctx, val);

      JS_FreeContext(ctx);
      JS_FreeRuntime(rt);
    }
  }
}
