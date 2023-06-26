use std::fmt;

#[allow(clippy::needless_return)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");

    println!("cargo:rerun-if-env-changed=CARGO_CFG_DOCS_RS");
    if std::env::var("CARGO_CFG_DOCS_RS").is_ok() {
        use_feature("outline_syscalls");
        return;
    }

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "linux" {
        return;
    }

    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    if std::env::var("CARGO_CFG_OUTLINE_SYSCALLS").is_ok()
        && std::env::var("CARGO_CFG_FORCE_INLINE_SYSCALLS").is_ok()
    {
        panic!("`--cfg outline_syscalls` and `--cfg force_inline_syscalls` are mutually exclusive");
    }

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let pointer_width = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    let endian = std::env::var("CARGO_CFG_TARGET_ENDIAN").unwrap();

    match arch.as_str() {
        "x86_64" if pointer_width == "64" && endian == "little" => main_x86_64(),
        "x86" if pointer_width == "32" && endian == "little" => main_x86(),
        "aarch64" if pointer_width == "64" && endian == "little" => main_aarch64(),
        "arm" if pointer_width == "32" && endian == "little" => main_arm(),
        "riscv32" if pointer_width == "32" && endian == "little" => main_riscv(),
        "riscv64" if pointer_width == "64" && endian == "little" => main_riscv(),
        "powerpc" if pointer_width == "32" && endian == "big" => main_powerpc(),
        "powerpc64" if pointer_width == "64" => main_powerpc64(),
        "mips" if pointer_width == "32" => main_mips(),
        "mips64" if pointer_width == "64" => main_mips64(),
        "s390x" if pointer_width == "64" && endian == "big" => main_s390x(),
        "loongarch64" if pointer_width == "64" && endian == "little" => main_loongarch64(),
        _ => {
            panic!(
                "arch {} {}-bits {} endian unsupported",
                arch, pointer_width, endian
            );
        }
    }
}

fn main_x86_64() {
    if needs_outline_asm() {
        build_trampoline("x86_64")
    }
}

fn main_x86() {
    if needs_outline_asm() {
        build_trampoline("x86")
    }
}

fn main_aarch64() {
    if needs_outline_asm() {
        build_trampoline("aarch64")
    }
}

fn main_arm() {
    if has_thumb_mode() {
        use_feature("thumb_mode");
    }
    if needs_outline_asm() {
        build_trampoline("arm")
    }
}

fn main_riscv() {
    if needs_outline_asm() {
        build_trampoline("riscv")
    }
}

fn main_powerpc() {
    if needs_outline_asm() {
        build_trampoline("powerpc")
    }
}

fn main_powerpc64() {
    if needs_outline_asm() {
        build_trampoline("powerpc64")
    }
}

fn main_mips() {
    if needs_outline_asm() {
        build_trampoline("mips")
    }
}

fn main_mips64() {
    if needs_outline_asm() {
        build_trampoline("mips64")
    }
}

fn main_s390x() {
    if needs_outline_asm() {
        build_trampoline("s390x")
    }
}

fn main_loongarch64() {
    if needs_outline_asm() {
        build_trampoline("loongarch64")
    }
}

#[cfg(unix)]
fn has_thumb_mode() -> bool {
    use std::os::unix::prelude::OsStrExt;

    std::env::var_os("TARGET")
        .unwrap()
        .as_bytes()
        .starts_with(b"thumb")
}

#[cfg(not(unix))]
fn has_thumb_mode() -> bool {
    false
}

fn build_trampoline(arch: &str) {
    println!("cargo:rerun-if-changed=src/outline/{}.s", arch);
    cc::Build::new()
        .cargo_metadata(true)
        .emit_rerun_if_env_changed(true)
        .file(format!("src/outline/{}.s", arch))
        .compile("liblinux_syscalls_rs.a");
}

fn needs_outline_asm() -> bool {
    const STABLE_59: [&str; 5] = ["x86", "arm", "x86_64", "aarch64", "riscv64"];

    if std::env::var("CARGO_CFG_OUTLINE_SYSCALLS").is_ok() {
        use_feature("outline_syscalls");
        true
    } else {
        let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let version = rustc_version::version().unwrap();
        assert_eq!(version.major, 1);

        if version.minor >= 59 && STABLE_59.contains(&arch.as_str()) {
            return false;
        }

        let version_meta = rustc_version::version_meta().unwrap();

        if version_meta.channel == rustc_version::Channel::Nightly {
            use_feature("asm_experimental_arch");
            false
        } else {
            if std::env::var("CARGO_CFG_FORCE_INLINE_SYSCALLS").is_err() {
                use_feature("outline_syscalls");
            }
            true
        }
    }
}

fn use_feature<T: fmt::Display>(feat: T) {
    println!("cargo:rustc-cfg={}", feat);
}
