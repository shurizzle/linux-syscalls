use core::fmt;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    link_arg("-nostartfiles");
    link_arg("-nodefaultlibs");
    link_arg("-nostdlib");
}

fn link_arg<F>(arg: F)
where
    F: fmt::Display,
{
    println!("cargo:rustc-link-arg-bin=bare={}", arg);
}
