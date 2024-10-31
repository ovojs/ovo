use crate::ffi::*;

pub enum Source {
  Global(String),
  Module(String),
}

pub enum Flag {
  None,
  Async,
  CompileOnly,
  BacktraceBarrier,
  Strict,
}

impl Flag {
  pub fn to_raw_flag(&self) -> u32 {
    match self {
      Flag::None => 0,
      Flag::Async => JS_EVAL_FLAG_ASYNC,
      Flag::CompileOnly => JS_EVAL_FLAG_COMPILE_ONLY,
      Flag::BacktraceBarrier => JS_EVAL_FLAG_BACKTRACE_BARRIER,
      Flag::Strict => JS_EVAL_FLAG_STRICT,
    }
  }
}

impl Source {
  pub fn to_raw_type(&self) -> u32 {
    match self {
      Source::Global(_) => JS_EVAL_TYPE_GLOBAL,
      Source::Module(_) => JS_EVAL_TYPE_MODULE,
    }
  }

  pub fn to_raw_code(&self) -> &String {
    match self {
      Source::Global(code) => code,
      Source::Module(code) => code,
    }
  }
}
