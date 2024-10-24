fn main() {
    println!("cargo:rsutc-link-search=./build");
    println!("cargo:rsutc-link-lib=bcos-c-sdk");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./build");
    println!("cargo:rustc-env=LIBRARY_PATH=./build");
}
