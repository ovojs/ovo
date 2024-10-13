#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use super::*;
  use std::ffi::CString;

  #[test]
  fn basic() {
    unsafe {
      let rt = JS_NewRuntime();
      let ctx = JS_NewContext(rt);

      let src = CString::new("40 + 2").unwrap();
      let name = CString::new("anon").unwrap();

      let val = JS_Eval(
        ctx,
        src.as_ptr(),
        src.count_bytes(),
        name.as_ptr(),
        JS_EVAL_TYPE_GLOBAL as i32,
      );

      let eq = JS_StrictEq(ctx, val, JS_NewInt32(ctx, 42));
      assert_eq!(eq, true as i32);

      JS_FreeValue(ctx, val);

      JS_FreeContext(ctx);
      JS_FreeRuntime(rt);
    }
  }
}
