use std::env;
use std::path::PathBuf;

/// Utility which sanitizes the comments
/// generated by bindgen so they aren't
/// mistakenly interpreted as tests:
/// https://github.com/rust-lang/rust-bindgen/issues/1313#issuecomment-1324102150
#[derive(Debug)]
struct CommentSanitizer;
impl bindgen::callbacks::ParseCallbacks for CommentSanitizer {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(format!("````ignore\n{}\n````", comment))
    }
}

fn main() {
    // Ask Cargo to link the library.
    let lib_path = format!("{}\\vendor\\winlibvicon", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=ViconDataStreamSDK_C");
    // println!("cargo:rustc-link-search=native=vendor//winlibvicon");
    // println!("cargo:rustc-link-lib=ViconDataStreamSDK_C");

    // Ask Cargo to invalidate the build cache
    // when the headers change.
    println!("cargo:rerun-if-changed=vendor//winlibvicon//CClient.h");

    // Generate C bindings.
    let bindings = bindgen::Builder::default()
        // Generate bindings for the header.
        // .header("vendor/libvicon/CClient.h")
        .header("vendor///winlibvicon///CClient.h")

        // Clean generated comments so they aren't
        // interpreted as tests.
        .parse_callbacks(Box::new(CommentSanitizer))
        // Generate bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libvicon.rs"))
        .expect("Couldn't write bindings!");
}
