fn main() {
    println!("cargo:rustc-link-search=./csdk");
    println!("cargo:rustc-link-lib=bcos-c-sdk-static");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=./csdk");
    println!("cargo:rustc-env=LIBRARY_PATH=./csdk");
}
