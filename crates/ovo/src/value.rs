use crate::handle::DropFromContext;
use crate::{Context, Value};
use ovo_quickjs::JS_FreeValue;

impl Value {}

impl DropFromContext for Value {
  fn drop_from_context(&mut self, ctx: &Context) {
    unsafe { JS_FreeValue(ctx.inner.as_ptr(), self.0) };
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{BigInt64, Bool, Float64, Int32, Int64, Runtime};
  use std::i64;

  #[test]
  fn value_convertion() {
    let rt = &Runtime::new(Default::default());
    let ctx = &Context::new(rt);

    let boolean = Bool::new(ctx, true);
    assert_eq!(boolean.value(ctx).unwrap(), true);

    let int32 = Int32::new(ctx, 114514);
    assert_eq!(int32.value(ctx).unwrap(), 114514);
    let int64 = Int64::new(ctx, i64::MAX);
    assert_ne!(int64.value(ctx).unwrap(), i64::MAX);
    let bigint64 = BigInt64::new(ctx, i64::MAX);
    assert_eq!(bigint64.value(ctx).unwrap(), i64::MAX);

    let float64 = Float64::new(ctx, 3.1415926);
    assert_eq!(float64.value(ctx).unwrap(), 3.1415926);
  }
}
