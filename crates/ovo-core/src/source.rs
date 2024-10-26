use crate::quickjs::*;

pub enum Source<'a> {
  Global(&'a str),
  Module(&'a str),
}

pub enum Flag {
  None,
  Async,
  CompileOnly,
  BacktraceBarrier,
  Strict,
  Strip,
}

impl Flag {
  pub fn to_raw_flag(&self) -> u32 {
    match self {
      Flag::None => 0,
      Flag::Async => JS_EVAL_FLAG_ASYNC,
      Flag::CompileOnly => JS_EVAL_FLAG_COMPILE_ONLY,
      Flag::BacktraceBarrier => JS_EVAL_FLAG_BACKTRACE_BARRIER,
      Flag::Strict => JS_EVAL_FLAG_STRICT,
      Flag::Strip => JS_EVAL_FLAG_STRIP,
    }
  }
}

impl<'a> Source<'a> {
  pub fn to_raw_type(&self) -> u32 {
    match self {
      Source::Global(_) => JS_EVAL_TYPE_GLOBAL,
      Source::Module(_) => JS_EVAL_TYPE_MODULE,
    }
  }

  pub fn to_raw_code(&self) -> &str {
    match self {
      Source::Global(code) => code,
      Source::Module(code) => code,
    }
  }
}
