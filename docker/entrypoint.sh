#!/bin/bash

eval "$(cat /env.sh | sed 's/^/export /g')"

set -eux

if [ $# -eq 0 ]; then
	exec /bin/bash
else
	exec "$@"
fi
