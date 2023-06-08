FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ADD /scripts/install.sh /install.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh $ARCH && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
CMD ["/bin/bash"]

ENV CC_i686_unknown_linux_gnu='i686-linux-gnu-gcc'
ENV CXX_i686_unknown_linux_gnu='i686-linux-gnu-g++'
ENV AR_i686_unknown_linux_gnu='i686-linux-gnu-ar'
ENV CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_LINKER='i686-linux-gnu-gcc'
ENV CARGO_TARGET_I686_UNKNOWN_LINUX_GNU_RUNNER='qemu-i386 -L /usr/i686-linux-gnu'
ENV CC_i686_unknown_linux_musl='i686-linux-musl-gcc'
ENV CXX_i686_unknown_linux_musl='i686-linux-musl-g++'
ENV AR_i686_unknown_linux_musl='i686-linux-musl-ar'
ENV CARGO_TARGET_I686_UNKNOWN_LINUX_MUSL_LINKER='i686-linux-musl-gcc'
ENV CARGO_TARGET_I686_UNKNOWN_LINUX_MUSL_RUNNER='qemu-i386 -L /usr/i686-linux-musl'
ENV CC_i586_unknown_linux_gnu='i686-linux-gnu-gcc'
ENV CXX_i586_unknown_linux_gnu='i686-linux-gnu-g++'
ENV AR_i586_unknown_linux_gnu='i686-linux-gnu-ar'
ENV CARGO_TARGET_I586_UNKNOWN_LINUX_GNU_LINKER='i686-linux-gnu-gcc'
ENV CARGO_TARGET_I586_UNKNOWN_LINUX_GNU_RUNNER='qemu-i386 -L /usr/i686-linux-gnu'
ENV CC_i586_unknown_linux_musl='i686-linux-musl-gcc'
ENV CXX_i586_unknown_linux_musl='i686-linux-musl-g++'
ENV AR_i586_unknown_linux_musl='i686-linux-musl-ar'
ENV CARGO_TARGET_I586_UNKNOWN_LINUX_MUSL_LINKER='i686-linux-musl-gcc'
ENV CARGO_TARGET_I586_UNKNOWN_LINUX_MUSL_RUNNER='qemu-i386 -L /usr/i686-linux-musl'
