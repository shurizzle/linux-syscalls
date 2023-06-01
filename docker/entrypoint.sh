#!/bin/sh

set -eux
if [ -e /env.sh ]; then
	. /env.sh
fi

if [ $# -eq 0 ]; then
	exec /bin/bash
else
	exec "$@"
fi
