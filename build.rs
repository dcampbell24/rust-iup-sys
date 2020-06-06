fn main() {
//    println!("cargo:rustc-cfg=warningABI"); // uncommenting at line's beginning enables accessibility of those declarations, that must be used with care for ABI compatibility of IUP binary used
    println!("cargo:rustc-flags=-l iup");
}
