#!/bin/bash

eval "$(grep -vP '^\s*($|#)' /env.sh | sed -e '/^#/d;/^\s*$/d' -e "s/'/'\\\''/g" -e "s/=\(.*\)/='\1'/g" -e 's/^/export /')"

set -eux

if [ $# -eq 0 ]; then
	exec /bin/bash
else
	exec "$@"
fi
