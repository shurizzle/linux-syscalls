#!/bin/bash

set -eux

rustup toolchain install 1.40.0
rustup target add \
	aarch64-unknown-linux-gnu \
	arm-unknown-linux-gnueabi \
	mips-unknown-linux-gnu \
	mipsel-unknown-linux-gnu \
	mips64-unknown-linux-gnuabi64 \
	mips64el-unknown-linux-gnuabi64 \
	powerpc-unknown-linux-gnu \
	powerpc64-unknown-linux-gnu \
	powerpc64le-unknown-linux-gnu \
	s390x-unknown-linux-gnu \
	i686-unknown-linux-gnu \
	x86_64-unknown-linux-gnux32 \
	--toolchain 1.40.0
rustup toolchain install 1.42.0 -t riscv64gc-unknown-linux-gnu
rustup toolchain install nightly -t loongarch64-unknown-linux-gnu

#	powerpc-unknown-linux-gnu \
for target in \
	aarch64-unknown-linux-gnu \
	arm-unknown-linux-gnueabi \
	mips-unknown-linux-gnu \
	mipsel-unknown-linux-gnu \
	mips64-unknown-linux-gnuabi64 \
	mips64el-unknown-linux-gnuabi64 \
	powerpc64-unknown-linux-gnu \
	powerpc64le-unknown-linux-gnu \
	s390x-unknown-linux-gnu \
	i686-unknown-linux-gnu \
	x86_64-unknown-linux-gnux32 \
	x86_64-unknown-linux-gnu; do
	RUSTFLAGS="--cfg docs_rs" cargo +1.40.0 doc --no-deps --target "$target"
done

RUSTFLAGS="--cfg docs_rs" cargo +1.42.0 doc --no-deps --target riscv64gc-unknown-linux-gnu

RUSTFLAGS="--cfg docs_rs" cargo +nightly doc --no-deps --target loongarch64-unknown-linux-gnu

# 	riscv32gc-unknown-linux-gnu \
for target in \
	armeb-unknown-linux-gnueabi; do
	RUSTFLAGS="--cfg docs_rs" cargo +nightly -Z build-std=core doc --no-deps --target "$target"
done
