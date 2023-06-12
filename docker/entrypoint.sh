#!/bin/bash

QEMU_UNAME="$(uname -r)"
export QEMU_UNAME

if [ $# -eq 0 ]; then
	set -- /bin/bash
fi

exec "$@"
