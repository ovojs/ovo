use std::{env, path::PathBuf};

fn main() {
  let lib_name = "quickjs";
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  let qjs_path = PathBuf::from("quickjs");

  cc::Build::new()
    .files(
      [
        "cutils.c",
        "libbf.c",
        "libregexp.c",
        "libunicode.c",
        "quickjs.c",
      ]
      .iter()
      .map(|f| qjs_path.join(f)),
    )
    // Flags below are used by the official Makefile.
    .flag_if_supported("-Wchar-subscripts")
    .flag_if_supported("-Wno-array-bounds")
    .flag_if_supported("-Wno-format-truncation")
    .flag_if_supported("-Wno-missing-field-initializers")
    .flag_if_supported("-Wno-sign-compare")
    .flag_if_supported("-Wno-unused-parameter")
    .flag_if_supported("-Wundef")
    .flag_if_supported("-Wuninitialized")
    .flag_if_supported("-Wunused")
    .flag_if_supported("-Wwrite-strings")
    .flag_if_supported("-funsigned-char")
    // Flags below are used to supress warnings on some platforms.
    .flag_if_supported("-Wno-cast-function-type")
    .flag_if_supported("-Wno-implicit-fallthrough")
    .flag_if_supported("-Wno-enum-conversion")
    .opt_level(2)
    .compile(&lib_name);

  bindgen::Builder::default()
    .headers(
      ["quickjs.h"]
        .iter()
        .map(|f| qjs_path.join(f).into_os_string().into_string().unwrap()),
    )
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .blocklist_item("FP_(NAN|INFINITE|ZERO|SUBNORMAL|NORMAL)")
    .generate()
    .expect("bindgen: failed to generate.")
    .write_to_file(out_path.join("bindgen.rs"))
    .expect("bindgen: failed to write bindings.rs");
}
