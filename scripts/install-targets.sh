#!/bin/bash

set -eux

install_x86_64() {
	local -a targets
	targets=(x86_64-unknown-linux-{gnu,musl})

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_x86() {
	local -a targets
	targets=(i{5,6}86-unknown-linux-{gnu,musl})

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_arm() {
	local -a targets
	targets=(
		{arm,armv7}-unknown-linux-{gnu,musl}eabi{,hf}
		armv5te-unknown-linux-{gnu,musl}eabi
		thumbv7neon-unknown-linux-gnueabihf
	)

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_aarch64() {
	local -a targets
	targets=(aarch64-unknown-linux-{gnu,musl})

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_riscv64() {
	rustup target add riscv64gc-unknown-linux-gnu --toolchain stable
	rustup target add riscv64gc-unknown-linux-gnu --toolchain 1.42.0
	rustup target add riscv64gc-unknown-linux-gnu --toolchain nightly
}

install_loongarch64() {
	rustup target add loongarch64-unknown-linux-gnu --toolchain nightly
}

install_powerpc() {
	rustup target add powerpc-unknown-linux-gnu --toolchain stable
	rustup target add powerpc-unknown-linux-gnu --toolchain 1.40.0
	rustup target add powerpc-unknown-linux-gnu --toolchain nightly
}

install_powerpc64() {
	local -a targets
	targets=(powerpc64{,le}-unknown-linux-gnu)

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_mips() {
	local -a targets
	targets=(mips{,el}-unknown-linux-{gnu,musl})

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_mips64() {
	local -a targets
	targets=(mips64{,el}-unknown-linux-{gnu,musl}abi64)

	rustup target add "${targets[@]}" --toolchain stable
	rustup target add "${targets[@]}" --toolchain 1.40.0
	rustup target add "${targets[@]}" --toolchain nightly
}

install_s390x() {
	local -a targets

	rustup target add s390x-unknown-linux-gnu --toolchain stable
	rustup target add s390x-unknown-linux-gnu --toolchain 1.40.0
	rustup target add s390x-unknown-linux-gnu --toolchain nightly
}

if [ $# -eq 0 ]; then
	set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
		mips64 s390x
fi

while [ $# -ne 0 ]; do
	"install_$1"
	shift
done
