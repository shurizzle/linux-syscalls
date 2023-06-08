#!/bin/bash

set -eux

ARCH="$1"
shift

set_env() {
	local triple="$1" \
		prefix="$2" \
		runner="${3:-}" \
		lcase_triple \
		ucase_triple

	lcase_triple="$(printf %s "$triple" | tr '-' '_')"
	ucase_triple="$(printf %s "$triple" | tr 'a-z-' 'A-Z_')"

	if [ -n "${4:-}" ]; then
		echo "CC_${lcase_triple}=${4}"
	else
		echo "CC_${lcase_triple}=${prefix}gcc"
	fi
	echo "CXX_${lcase_triple}=${prefix}g++"
	echo "AR_${lcase_triple}=${prefix}ar"
	if [ -n "${4:-}" ]; then
		echo "CARGO_TARGET_${ucase_triple}_LINKER=${4}"
	else
		echo "CARGO_TARGET_${ucase_triple}_LINKER=${prefix}gcc"
	fi
	if [ -n "$runner" ]; then
		echo "CARGO_TARGET_${ucase_triple}_RUNNER=$runner"
	fi
}

env_x86_64() {
	local libc triple

	for libc in gnu musl; do
		triple="x86_64-unknown-linux-${libc}"

		set_env "$triple" \
			x86_64-linux-gnu-
	done
}

env_loongarch64() {
	local triple="loongarch64-unknown-linux-gnu"
	set_env "$triple" \
		/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu- \
		"qemu-loongarch64 -E LD_LIBRARY_PATH=/lib:/lib64 -L /opt/loongarch64-unknown-linux-gnu/target/usr"
}

env_x86() {
	local arch libc triple

	for arch in i686 i586; do
		for libc in gnu musl; do
			triple="${arch}-unknown-linux-${libc}"

			set_env "$triple" \
				"i686-linux-${libc}-" \
				"qemu-i386 -L /usr/i686-linux-${libc}"
		done
	done
}

env_arm() {
	local arch libc

	for arch in arm armv7; do
		for libc in gnu musl; do
			set_env "${arch}-unknown-linux-${libc}eabi" \
				"arm-linux-${libc}eabi-" \
				"qemu-arm -L /usr/arm-linux-${libc}eabi"
		done
	done

	set_env armv5te-unknown-linux-gnueabi \
		arm-linux-gnueabi- \
		"qemu-arm -L /usr/arm-linux-gnueabi"

	set_env armv5te-unknown-linux-musleabi \
		arm-linux-musleabi- \
		"qemu-arm -L /usr/arm-linux-musleabi" \
		/armv5te-musl.sh

	for arch in arm armv7 thumbv7neon; do
		for libc in gnu musl; do
			set_env "${arch}-unknown-linux-${libc}eabihf" \
				"arm-linux-${libc}eabihf-" \
				"qemu-arm -L /usr/arm-linux-${libc}eabihf"
		done
	done
}

env_aarch64() {
	set_env "aarch64-unknown-linux-gnu" \
		"aarch64-linux-gnu-" \
		"qemu-aarch64 -L /usr/aarch64-linux-gnu"

	set_env "aarch64-unknown-linux-musl" \
		"aarch64-linux-musl-" \
		"qemu-aarch64 -L /usr/aarch64-linux-musl" \
		/aarch64-musl.sh
}

env_riscv64() {
	local libc triple

	for libc in gnu musl; do
		triple="riscv64gc-unknown-linux-${libc}"

		set_env "$triple" \
			"riscv64-linux-${libc}-" \
			"qemu-riscv64 -L /usr/riscv64-linux-${libc}"
	done
}

env_powerpc() {
	local libc

	for libc in gnu musl; do
		set_env "powerpc-unknown-linux-${libc}" \
			"powerpc-linux-${libc}-" \
			"qemu-ppc -L /usr/powerpc-linux-${libc}"
	done
}

env_powerpc64() {
	local libc

	for libc in gnu musl; do
		set_env "powerpc64-unknown-linux-${libc}" \
			"powerpc64-linux-${libc}-" \
			"qemu-ppc64 -L /usr/powerpc64-linux-${libc}"

		set_env "powerpc64le-unknown-linux-${libc}" \
			"powerpc64le-linux-${libc}-" \
			"qemu-ppc64le -L /usr/powerpc64le-linux-${libc}"
	done
}

env_mips() {
	local libc arch

	for arch in mips mipsel; do
		for libc in gnu musl; do
			set_env "${arch}-unknown-linux-${libc}" \
				"${arch}-linux-${libc}-" \
				"qemu-${arch} -L /usr/${arch}-linux-${libc}"
		done
	done
}

env_mips64() {
	local libc arch

	for arch in mips64 mips64el; do
		set_env "${arch}-unknown-linux-gnuabi64" \
			"${arch}-linux-gnuabi64-" \
			"qemu-${arch} -L /usr/${arch}-linux-gnuabi64"

		set_env "${arch}-unknown-linux-muslabi64" \
			"${arch}-linux-muslabi64-" \
			"qemu-${arch} -L /usr/${arch}-linux-muslabi64" \
			"/${arch}-musl.sh"
	done
}

env_s390x() {
	local libc

	for libc in gnu musl; do
		set_env "s390x-unknown-linux-${libc}" \
			"s390x-linux-${libc}-" \
			"qemu-s390x -L /usr/s390x-linux-${libc}"
	done
}

"env_${ARCH}"
