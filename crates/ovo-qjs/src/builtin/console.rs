use crate::context::Context;
use crate::extension::CallArgs;
use crate::function;
use crate::value::{Int32, Value};

function!(js_log |ctx: &Context, args: CallArgs| -> Value {

  Int32::new(ctx, 42).into()
});
