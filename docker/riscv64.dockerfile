FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ENV PATH="${PATH}:/rust/bin"
ENV RUST_COMPILER_RT_ROOT=/opt/compiler-rt

ADD /scripts/install.sh /install.sh
ADD /docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh && \
  chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh "${ARCH}" && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/bash"]

ENV CC_riscv64gc_unknown_linux_gnu='riscv64-linux-gnu-gcc'
ENV CXX_riscv64gc_unknown_linux_gnu='riscv64-linux-gnu-g++'
ENV AR_riscv64gc_unknown_linux_gnu='riscv64-linux-gnu-ar'
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER='riscv64-linux-gnu-gcc'
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER='qemu-riscv64 -L /usr/riscv64-linux-gnu'
ENV CC_riscv64gc_unknown_linux_musl='riscv64-linux-musl-gcc'
ENV CXX_riscv64gc_unknown_linux_musl='riscv64-linux-musl-g++'
ENV AR_riscv64gc_unknown_linux_musl='riscv64-linux-musl-ar'
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_LINKER='riscv64-linux-musl-gcc'
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_MUSL_RUNNER='qemu-riscv64 -L /usr/riscv64-linux-musl'
