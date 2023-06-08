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

expected() {
	echo "$1"
	uname -r | cut -d- -f1
	echo 69
}

cargo_build() {
	cross +"$1" -vvv build --target "$2" --example kernel_exit --release
}

cargo_run() {
	cross +"$1" run --target "$2" --example kernel_exit --release
}

cargo_test() {
	local expected
	expected="$(expected "$3")"

	rm -rf Cargo.lock target
	cargo_build "$@"
	set +e
	cargo_run "$@"
	echo $?
	set -e
	[ "$(output_and_exit_code cargo_run "$@")" = "$expected" ]
}

test_nightly() {
	rm -rf Cargo.lock target
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test nightly "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test nightly "$@"
}

test_stable() {
	rm -rf Cargo.lock target
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test 1.59.0 "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.40.0}" "$@"
	test_nightly "$@"
}

test_unstable() {
	rm -rf Cargo.lock target
	RUSTFLAGS="--cfg outline_syscalls" cargo_test 1.59.0 "$@"
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
