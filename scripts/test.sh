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
	cargo +"$1" build --target "$2" --example kernel_exit --release
}

cargo_run() {
	cargo +"$1" run --target "$2" --example kernel_exit --release
}

cargo_test() {
	local expected target
	expected="$(expected "$3")"
	target="target/$2"

	cargo_build "$@"
	set +e
	cargo_run "$@"
	echo $?
	set -e
	[ "$(output_and_exit_code cargo_run "$@")" = "$expected" ]
	rm -r "$target"
}

test_nightly() {
	rm -rf "target/$1"
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test nightly "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test nightly "$@"
}

test_stable() {
	rm -rf "target/$1"
	RUSTFLAGS="--cfg force_inline_syscalls" cargo_test 1.59.0 "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.40.0}" "$@"
	test_nightly "$@"
}

test_unstable() {
	rm -rf "target/$1"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test 1.59.0 "$@"
	RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.40.0}" "$@"
	test_nightly "$@"
}

test_x86_64() {
	local libc triple

	for libc in gnu; do
		triple="x86_64-unknown-linux-${libc}"
		test_stable "$triple" x86_64
	done
}

test_x86() {
	local arch libc triple

	for arch in i686 i586; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}"
			test_stable "$triple" x86
		done
	done
}

test_arm() {
	local arch libc triple

	for arch in arm armv5te armv7; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}eabi"
			test_stable "$triple" arm
		done
	done

	for arch in arm armv7 thumbv7neon; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}eabihf"
			test_stable "$triple" arm
		done
	done
}

test_aarch64() {
	local libc triple
	for libc in gnu; do
		triple="aarch64-unknown-linux-${libc}"
		test_stable "$triple" aarch64
	done
}

test_riscv64() {
	local libc triple
	for libc in gnu; do
		triple="riscv64gc-unknown-linux-${libc}"
		test_stable "$triple" riscv64 1.42.0
	done
}

test_loongarch64() {
	test_nightly "loongarch64-unknown-linux-gnu" loongarch64
}

test_powerpc() {
	local libc
	for libc in gnu; do
		test_unstable "powerpc-unknown-linux-gnu" powerpc
	done
}

test_powerpc64() {
	local arch libc
	for arch in powerpc64 powerpc64le; do
		for libc in gnu; do
			test_unstable "${arch}-unknown-linux-gnu" powerpc64
		done
	done
}

test_mips() {
	local arch libc
	for arch in mips mipsel; do
		for libc in gnu; do
			test_unstable "${arch}-unknown-linux-gnu" mips
		done
	done
}

test_mips64() {
	local arch libc
	for arch in mips64 mips64el; do
		for libc in gnu; do
			test_unstable "${arch}-unknown-linux-gnuabi64" mips64
		done
	done
}

test_s390x() {
	local libc
	for libc in gnu; do
		test_unstable "s390x-unknown-linux-gnu" s390x
	done
}

# TODO: riscv32

test_"$1"
