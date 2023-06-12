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

ENV CC_mips64_unknown_linux_gnuabi64='mips64-linux-gnuabi64-gcc'
ENV CXX_mips64_unknown_linux_gnuabi64='mips64-linux-gnuabi64-g++'
ENV AR_mips64_unknown_linux_gnuabi64='mips64-linux-gnuabi64-ar'
ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_GNUABI64_LINKER='mips64-linux-gnuabi64-gcc'
ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_GNUABI64_RUNNER='qemu-mips64 -L /usr/mips64-linux-gnuabi64'
ENV CC_mips64_unknown_linux_muslabi64='/mips64-musl.sh'
ENV CXX_mips64_unknown_linux_muslabi64='mips64-linux-muslabi64-g++'
ENV AR_mips64_unknown_linux_muslabi64='mips64-linux-muslabi64-ar'
ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_MUSLABI64_LINKER='/mips64-musl.sh'
ENV CARGO_TARGET_MIPS64_UNKNOWN_LINUX_MUSLABI64_RUNNER='qemu-mips64 -L /usr/mips64-linux-muslabi64'
ENV CC_mips64el_unknown_linux_gnuabi64='mips64el-linux-gnuabi64-gcc'
ENV CXX_mips64el_unknown_linux_gnuabi64='mips64el-linux-gnuabi64-g++'
ENV AR_mips64el_unknown_linux_gnuabi64='mips64el-linux-gnuabi64-ar'
ENV CARGO_TARGET_MIPS64EL_UNKNOWN_LINUX_GNUABI64_LINKER='mips64el-linux-gnuabi64-gcc'
ENV CARGO_TARGET_MIPS64EL_UNKNOWN_LINUX_GNUABI64_RUNNER='qemu-mips64el -L /usr/mips64el-linux-gnuabi64'
ENV CC_mips64el_unknown_linux_muslabi64='/mips64el-musl.sh'
ENV CXX_mips64el_unknown_linux_muslabi64='mips64el-linux-muslabi64-g++'
ENV AR_mips64el_unknown_linux_muslabi64='mips64el-linux-muslabi64-ar'
ENV CARGO_TARGET_MIPS64EL_UNKNOWN_LINUX_MUSLABI64_LINKER='/mips64el-musl.sh'
ENV CARGO_TARGET_MIPS64EL_UNKNOWN_LINUX_MUSLABI64_RUNNER='qemu-mips64el -L /usr/mips64el-linux-muslabi64'
