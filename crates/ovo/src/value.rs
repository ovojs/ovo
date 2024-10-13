use crate::context::{Context, DropFromContext};
use ovo_quickjs::*;
use std::ffi::c_int;

pub struct Value(pub(crate) JSValue);

impl Value {
  #[inline(always)]
  pub fn from_bool(ctx: &Context, val: bool) -> Self {
    unsafe { Self(JS_NewBool(ctx.inner.as_ptr(), val as c_int)) }
  }

  #[inline(always)]
  pub fn from_i32(ctx: &Context, val: i32) -> Self {
    unsafe { Self(JS_NewInt32(ctx.inner.as_ptr(), val)) }
  }

  #[inline(always)]
  pub fn to_bool(&self, ctx: &Context) -> Option<bool> {
    unsafe { Some(JS_ToBool(ctx.inner.as_ptr(), self.0) != 0) }
  }

  #[inline(always)]
  pub fn to_i32(&self, ctx: &Context) -> Option<i32> {
    let mut val = 0;
    let ret = unsafe { JS_ToInt32(ctx.inner.as_ptr(), &mut val as *mut i32, self.0) };
    if ret == 0 {
      Some(val)
    } else {
      None
    }
  }

  #[inline(always)]
  pub fn struct_eq(&self, ctx: &Context, val: &Value) -> bool {
    unsafe { JS_StrictEq(ctx.inner.as_ptr(), self.0, val.0) != 0 }
  }
}

impl DropFromContext for Value {
  fn drop_from_context(&mut self, ctx: &Context) {
    unsafe { JS_FreeValue(ctx.inner.as_ptr(), self.0) };
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Runtime;

  #[test]
  fn basic() {
    let rt = &Runtime::new(Default::default());
    let ctx = &Context::new(rt);
    let val_bool = Value::from_bool(ctx, true);
    assert_eq!(val_bool.to_bool(ctx).unwrap(), true);
    let val_i32 = Value::from_i32(ctx, 114514);
    assert_eq!(val_i32.to_i32(ctx).unwrap(), 114514);
  }
}
