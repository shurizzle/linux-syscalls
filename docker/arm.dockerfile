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

ENV CC_arm_unknown_linux_gnueabi='arm-linux-gnueabi-gcc'
ENV CXX_arm_unknown_linux_gnueabi='arm-linux-gnueabi-g++'
ENV AR_arm_unknown_linux_gnueabi='arm-linux-gnueabi-ar'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER='arm-linux-gnueabi-gcc'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_RUNNER='qemu-arm -L /usr/arm-linux-gnueabi'
ENV CC_arm_unknown_linux_musleabi='arm-linux-musleabi-gcc'
ENV CXX_arm_unknown_linux_musleabi='arm-linux-musleabi-g++'
ENV AR_arm_unknown_linux_musleabi='arm-linux-musleabi-ar'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABI_LINKER='arm-linux-musleabi-gcc'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABI_RUNNER='qemu-arm -L /usr/arm-linux-musleabi'
ENV CC_armv7_unknown_linux_gnueabi='arm-linux-gnueabi-gcc'
ENV CXX_armv7_unknown_linux_gnueabi='arm-linux-gnueabi-g++'
ENV AR_armv7_unknown_linux_gnueabi='arm-linux-gnueabi-ar'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_LINKER='arm-linux-gnueabi-gcc'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUNNER='qemu-arm -L /usr/arm-linux-gnueabi'
ENV CC_armv7_unknown_linux_musleabi='arm-linux-musleabi-gcc'
ENV CXX_armv7_unknown_linux_musleabi='arm-linux-musleabi-g++'
ENV AR_armv7_unknown_linux_musleabi='arm-linux-musleabi-ar'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_LINKER='arm-linux-musleabi-gcc'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_RUNNER='qemu-arm -L /usr/arm-linux-musleabi'
ENV CC_armv5te_unknown_linux_gnueabi='arm-linux-gnueabi-gcc'
ENV CXX_armv5te_unknown_linux_gnueabi='arm-linux-gnueabi-g++'
ENV AR_armv5te_unknown_linux_gnueabi='arm-linux-gnueabi-ar'
ENV CARGO_TARGET_ARMV5TE_UNKNOWN_LINUX_GNUEABI_LINKER='arm-linux-gnueabi-gcc'
ENV CARGO_TARGET_ARMV5TE_UNKNOWN_LINUX_GNUEABI_RUNNER='qemu-arm -L /usr/arm-linux-gnueabi'
ENV CC_armv5te_unknown_linux_musleabi='/armv5te-musl.sh'
ENV CXX_armv5te_unknown_linux_musleabi='arm-linux-musleabi-g++'
ENV AR_armv5te_unknown_linux_musleabi='arm-linux-musleabi-ar'
ENV CARGO_TARGET_ARMV5TE_UNKNOWN_LINUX_MUSLEABI_LINKER='/armv5te-musl.sh'
ENV CARGO_TARGET_ARMV5TE_UNKNOWN_LINUX_MUSLEABI_RUNNER='qemu-arm -L /usr/arm-linux-musleabi'
ENV CC_arm_unknown_linux_gnueabihf='arm-linux-gnueabihf-gcc'
ENV CXX_arm_unknown_linux_gnueabihf='arm-linux-gnueabihf-g++'
ENV AR_arm_unknown_linux_gnueabihf='arm-linux-gnueabihf-ar'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER='arm-linux-gnueabihf-gcc'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-gnueabihf'
ENV CC_arm_unknown_linux_musleabihf='arm-linux-musleabihf-gcc'
ENV CXX_arm_unknown_linux_musleabihf='arm-linux-musleabihf-g++'
ENV AR_arm_unknown_linux_musleabihf='arm-linux-musleabihf-ar'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER='arm-linux-musleabihf-gcc'
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-musleabihf'
ENV CC_armv7_unknown_linux_gnueabihf='arm-linux-gnueabihf-gcc'
ENV CXX_armv7_unknown_linux_gnueabihf='arm-linux-gnueabihf-g++'
ENV AR_armv7_unknown_linux_gnueabihf='arm-linux-gnueabihf-ar'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER='arm-linux-gnueabihf-gcc'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-gnueabihf'
ENV CC_armv7_unknown_linux_musleabihf='arm-linux-musleabihf-gcc'
ENV CXX_armv7_unknown_linux_musleabihf='arm-linux-musleabihf-g++'
ENV AR_armv7_unknown_linux_musleabihf='arm-linux-musleabihf-ar'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER='arm-linux-musleabihf-gcc'
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-musleabihf'
ENV CC_thumbv7neon_unknown_linux_gnueabihf='arm-linux-gnueabihf-gcc'
ENV CXX_thumbv7neon_unknown_linux_gnueabihf='arm-linux-gnueabihf-g++'
ENV AR_thumbv7neon_unknown_linux_gnueabihf='arm-linux-gnueabihf-ar'
ENV CARGO_TARGET_THUMBV7NEON_UNKNOWN_LINUX_GNUEABIHF_LINKER='arm-linux-gnueabihf-gcc'
ENV CARGO_TARGET_THUMBV7NEON_UNKNOWN_LINUX_GNUEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-gnueabihf'
ENV CC_thumbv7neon_unknown_linux_musleabihf='arm-linux-musleabihf-gcc'
ENV CXX_thumbv7neon_unknown_linux_musleabihf='arm-linux-musleabihf-g++'
ENV AR_thumbv7neon_unknown_linux_musleabihf='arm-linux-musleabihf-ar'
ENV CARGO_TARGET_THUMBV7NEON_UNKNOWN_LINUX_MUSLEABIHF_LINKER='arm-linux-musleabihf-gcc'
ENV CARGO_TARGET_THUMBV7NEON_UNKNOWN_LINUX_MUSLEABIHF_RUNNER='qemu-arm -L /usr/arm-linux-musleabihf'
