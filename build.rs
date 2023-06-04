use std::{fmt, io::Write};

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
        "riscv64" if pointer_width == "64" && endian == "little" => main_riscv64(),
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
    if needs_outline_asm("ud2", true) {
        build_trampoline("x86_64")
    }
}

fn main_x86() {
    if needs_outline_asm("ud2", true) {
        build_trampoline("x86")
    }
}

fn main_aarch64() {
    if needs_outline_asm("udf #16", true) {
        build_trampoline("aarch64")
    }
}

fn main_arm() {
    if has_thumb_mode() {
        use_feature("thumb_mode");
    }
    if needs_outline_asm("udf #16", true) {
        build_trampoline("arm")
    }
}

fn main_riscv64() {
    if needs_outline_asm("unimpl", true) {
        build_trampoline("riscv64")
    }
}

fn main_powerpc64() {
    if needs_outline_asm("trap", true) {
        build_trampoline("powerpc64")
    }
}

fn main_mips() {
    if needs_outline_asm("teq $zero, $zero", true) {
        build_trampoline("mips")
    }
}

fn main_mips64() {
    if needs_outline_asm("teq $0, $0", true) {
        build_trampoline("mips64")
    }
}

fn main_s390x() {
    if needs_outline_asm("trap2", true) {
        build_trampoline("s390x")
    }
}

fn main_loongarch64() {
    if needs_outline_asm("break 1", true) {
        build_trampoline("loongarch64")
    }
}

fn has_thumb_mode() -> bool {
    !can_compile("#![feature(asm_experimental_arch)]\nextern crate core;\npub unsafe fn f() { ::core::arch::asm!(\"udf #16\", in(\"r7\") 0); }", true)
}

fn build_trampoline(arch: &str) {
    println!("cargo:rerun-if-changed=src/outline/{}.s", arch);
    cc::Build::new()
        .cargo_metadata(true)
        .emit_rerun_if_env_changed(true)
        .file(format!("src/outline/{}.s", arch))
        .compile("liblinux_syscalls_rs.a");
}

fn needs_outline_asm<T: fmt::Display>(instruction: T, metadata_only: bool) -> bool {
    if std::env::var("CARGO_CFG_OUTLINE_SYSCALLS").is_ok() {
        use_feature("outline_syscalls");
        true
    } else {
        match test_asm(instruction, metadata_only) {
            ASM::Stable => false,
            ASM::Nightly => {
                use_feature("asm_experimental_arch");
                false
            }
            ASM::Outline => {
                if std::env::var("CARGO_CFG_FORCE_INLINE_SYSCALLS").is_err() {
                    use_feature("outline_syscalls");
                }
                true
            }
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum ASM {
    Stable,
    Nightly,
    Outline,
}

fn test_asm<T: fmt::Display>(instruction: T, metadata_only: bool) -> ASM {
    let code = format!(
        "#![feature(asm_experimental_arch)]\nextern crate core;\npub unsafe fn f() {{ ::core::arch::asm!(\"{}\"); }}",
        instruction
    );

    if can_compile(&code[35..], metadata_only) {
        ASM::Stable
    } else if can_compile(code, metadata_only) {
        ASM::Nightly
    } else {
        ASM::Outline
    }
}

fn use_feature<T: fmt::Display>(feat: T) {
    println!("cargo:rustc-cfg={}", feat);
}

fn can_compile<T: AsRef<str>>(test: T, metadata_only: bool) -> bool {
    use std::process::Stdio;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let rustc = std::env::var("RUSTC").unwrap();
    let target = std::env::var("TARGET").unwrap();

    // Ditto 'RUSTC_WRAPPER'
    let wrapper =
        std::env::var("RUSTC_WRAPPER")
            .ok()
            .and_then(|w| if w.is_empty() { None } else { Some(w) });

    let mut cmd = if let Some(wrapper) = wrapper {
        let mut cmd = std::process::Command::new(wrapper);
        cmd.arg(rustc);
        cmd
    } else {
        std::process::Command::new(rustc)
    };

    cmd.arg("--crate-type=rlib")
        .tap(|cmd| {
            if metadata_only {
                cmd.arg("--emit=metadata")
            } else {
                cmd
            }
        })
        .arg("--target")
        .arg(target)
        .arg("--out-dir")
        .arg(out_dir);

    // Ditto RUSTFLAGS.
    if let Ok(rustflags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        if !rustflags.is_empty() {
            for arg in rustflags.split('\x1f') {
                cmd.arg(arg);
            }
        }
    }

    let mut child = cmd
        .arg("-")
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    writeln!(child.stdin.take().unwrap(), "{}", test.as_ref()).unwrap();

    child.wait().unwrap().success()
}

pub trait Tap: Sized {
    #[inline(always)]
    fn tap<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        f(self)
    }
}

impl<T> Tap for T {}
