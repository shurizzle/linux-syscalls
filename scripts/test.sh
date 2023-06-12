#!/bin/bash

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

cd "${SCRIPTPATH}/.."

set -eux

output_and_exit_code() {
	set +e
	"$@"
	echo $?
	set -e
}

# expected(toolchain, arch)
expected() {
	"${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
		/bin/bash -c "echo '$2' && echo \"\$QEMU_UNAME\" | cut -d- -f1 && echo 69"
}

# cargo_build(toolchain, arch, target)
cargo_build() {
	"${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
		cargo -vvv build \
		--target "$3" \
		--example kernel_exit \
		--release
}

# cargo_run(toolchain, arch, target)
cargo_run() {
	local runner
	runner="CARGO_TARGET_$(printf '%s' "$3" | tr 'a-z-' 'A-Z_')_RUNNER"

	"${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
		/bin/sh -c "exec \$${runner} /target/$3/release/examples/kernel_exit"
}

# cargo_clippy(toolchain, arch, target)
cargo_clippy() {
	"${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
		cargo -vvv clippy \
		--no-default-features \
		--target "$3" \
		-- -D warnings

	"${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
		cargo -vvv clippy \
		--features=bare \
		--target "$3" \
		-- -D warnings
}

# cargo_test(toolchain, target, arch)
cargo_test() {
	local expected

	rm -rf Cargo.lock target
	cargo_build "$1" "$3" "$2"
	expected="$(expected "$1" "$3")"
	local res
	res="$(output_and_exit_code cargo_run "$1" "$3" "$2")"
	[ "${res}" = "$expected" ]
}

# test_nightly(target, arch)
test_nightly() {
	cargo_clippy nightly "$2" "$1"
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test nightly "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test nightly "$@"
}

# test_stable(target, arch, toolchain?)
test_stable() {
	cargo_clippy stable "$2" "$1"
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test stable "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.40.0}" "$@"
	test_nightly "$@"
}

# test_unstable(target, arch, toolchain?)
test_unstable() {
	cargo_clippy stable "$2" "$1"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test stable "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.40.0}" "$@"
	test_nightly "$@"
}

test_x86_64() {
	local libc

	for libc in gnu musl; do
		test_stable "x86_64-unknown-linux-${libc}" x86_64
	done
}

test_x86() {
	local arch libc

	for arch in i686 i586; do
		for libc in gnu musl; do
			test_stable "${arch}-unknown-linux-${libc}" x86
		done
	done
}

test_arm() {
	local arch libc

	for arch in arm armv5te armv7; do
		for libc in gnu musl; do
			test_stable "${arch}-unknown-linux-${libc}eabi" arm
		done
	done

	for arch in arm armv7; do
		for libc in gnu musl; do
			test_stable "${arch}-unknown-linux-${libc}eabihf" arm
		done
	done

	test_stable thumbv7neon-unknown-linux-gnueabihf arm
	# test_stable thumbv7neon-unknown-linux-musleabihf arm
}

test_aarch64() {
	test_stable aarch64-unknown-linux-gnu aarch64
	test_stable aarch64-unknown-linux-musl aarch64
}

test_riscv64() {
	test_stable riscv64gc-unknown-linux-gnu riscv64 1.42.0
	# test_stable riscv64gc-unknown-linux-musl riscv64 1.42.0
}

test_loongarch64() {
	test_nightly loongarch64-unknown-linux-gnu loongarch64
}

test_powerpc() {
	test_unstable powerpc-unknown-linux-gnu powerpc
	# test_nightly powerpc-unknown-linux-musl powerpc
}

test_powerpc64() {
	test_unstable powerpc64-unknown-linux-gnu powerpc64
	test_unstable powerpc64le-unknown-linux-gnu powerpc64
	# test_nightly powerpc64-unknown-linux-musl powerpc64
	# test_nightly powerpc64le-unknown-linux-musl powerpc64
}

test_mips() {
	local arch libc
	for arch in mips mipsel; do
		for libc in gnu musl; do
			test_unstable "${arch}-unknown-linux-${libc}" mips
		done
	done
}

test_mips64() {
	local arch libc
	for arch in mips64 mips64el; do
		for libc in gnu musl; do
			test_unstable "${arch}-unknown-linux-${libc}abi64" mips64
		done
	done
}

test_s390x() {
	test_unstable s390x-unknown-linux-gnu s390x
	# test_nightly s390x-unknown-linux-musl s390x
}

# TODO: riscv32

if [ $# -eq 0 ]; then
	set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
		mips64 s390x
fi

while [ $# -ne 0 ]; do
	"test_$1"
	shift
done

# ./scripts/docker-run.sh nightly aarch64 /bin/sh -c 'cd bare && cargo -Z build-std=core -Z build-std-features=compiler-builtins-c,panic_immediate_abort build --target aarch64-unknown-linux-gnu --release && exec $CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER /target/aarch64-unknown-linux-gnu/release/bare'; echo $?
# ./scripts/docker-run.sh nightly x86_64 /bin/sh -c 'cd bare && cargo -Z build-std=core -Z build-std-features=compiler-builtins-c,panic_immediate_abort build --target x86_64-unknown-linux-gnu --release && exec $CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER /target/x86_64-unknown-linux-gnu/release/bare'; echo $?
# ./scripts/docker-run.sh nightly x86 /bin/sh -c 'cd bare && cargo -Z build-std=core -Z build-std-features=compiler-builtins-c,panic_immediate_abort build --target i686-unknown-linux-gnu --release && exec $CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_RUNNER /target/i686-unknown-linux-gnu/release/bare'; echo $?
# ./scripts/docker-run.sh nightly arm /bin/sh -c 'cd bare && cargo -Z build-std=core -Z build-std-features=compiler-builtins-c,panic_immediate_abort build -vvv --target arm-unknown-linux-gnueabi --release && exec $CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_RUNNER /target/arm-unknown-linux-gnueabi/release/bare'; echo $?
