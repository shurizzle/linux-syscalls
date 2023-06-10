FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ENV PATH="${PATH}:/rust/bin"
ENV RUST_COMPILER_RT_ROOT=/opt/compiler-rt

ADD /scripts/install.sh /install.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh "${ARCH}" && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
CMD ["/bin/bash"]

ENV CC_loongarch64_unknown_linux_gnu='/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc'
ENV CXX_loongarch64_unknown_linux_gnu='/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-g++'
ENV AR_loongarch64_unknown_linux_gnu='/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-ar'
ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER='/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc'
ENV CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_RUNNER='qemu-loongarch64 -E LD_LIBRARY_PATH=/lib:/lib64 -L /opt/loongarch64-unknown-linux-gnu/target/usr'
