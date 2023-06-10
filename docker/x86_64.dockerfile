FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ENV PATH="${PATH}:/rust/bin"

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

ENV CC_x86_64_unknown_linux_gnu='x86_64-linux-gnu-gcc'
ENV CXX_x86_64_unknown_linux_gnu='x86_64-linux-gnu-g++'
ENV AR_x86_64_unknown_linux_gnu='x86_64-linux-gnu-ar'
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER='x86_64-linux-gnu-gcc'
ENV CC_x86_64_unknown_linux_musl='x86_64-linux-gnu-gcc'
ENV CXX_x86_64_unknown_linux_musl='x86_64-linux-gnu-g++'
ENV AR_x86_64_unknown_linux_musl='x86_64-linux-gnu-ar'
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER='x86_64-linux-gnu-gcc'
