use ovo_qjs::{function, CallArgs, Context, Int32, Value};

function!(add |ctx: &Context, args: CallArgs| ->  Value {
  let Some(arg0) = args.get(0).try_to_i32(ctx) else {
    return ctx.throw_type_error("expect i32");
  };
  let Some(arg1) = args.get(1).try_to_i32(ctx) else {
    return ctx.throw_type_error("expect i32");
  };
  Value::from(Int32::new(ctx, arg0 + arg1))
});

fn main() {}
