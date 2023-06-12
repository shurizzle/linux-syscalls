#!/bin/bash

set -eux

msrv=1.40.0
if [ $# -eq 0 ]; then
	msrv="$msrv 1.42.0"
elif [ "$1" = riscv64 ]; then
	msrv=1.42.0
fi

rustup update $msrv stable nightly --no-self-update
rustup component add clippy rust-src --toolchain nightly
rustup component add clippy --toolchain stable
rustup default stable

if [ -n "${GITHUB_ENV:-}" ]; then
	(
		echo 'CARGO_INCREMENTAL=0'
		echo 'CARGO_PROFILE_DEV_DEBUG=0'
		echo 'CARGO_PROFILE_TEST_DEBUG=0'
	) >"${GITHUB_ENV}"
fi
