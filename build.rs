use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=./mujs/");
    println!("cargo:rustc-link-lib=static=mujs");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./mujs/mujs.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
