extern crate bindgen;

fn main() {
    let generated = bindgen::builder()
        .header("src/cairo_xcb.h")
        .ctypes_prefix("libc")
        .no_unstable_rust()
        // TODO don't hardcode
        .clang_arg("-I")
        .clang_arg("/usr/include/cairo")
        .generate().unwrap();
    generated.write_to_file("src/cairo_xcb_gen.rs").unwrap();
    // TODO Static linking feature
}
