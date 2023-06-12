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

ENV CC_x86_64_unknown_linux_gnu='x86_64-linux-gnu-gcc'
ENV CXX_x86_64_unknown_linux_gnu='x86_64-linux-gnu-g++'
ENV AR_x86_64_unknown_linux_gnu='x86_64-linux-gnu-ar'
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER='x86_64-linux-gnu-gcc'
ENV CC_x86_64_unknown_linux_musl='x86_64-linux-gnu-gcc'
ENV CXX_x86_64_unknown_linux_musl='x86_64-linux-gnu-g++'
ENV AR_x86_64_unknown_linux_musl='x86_64-linux-gnu-ar'
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER='x86_64-linux-gnu-gcc'
