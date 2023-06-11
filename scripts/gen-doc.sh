#!/bin/bash

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

cd "${SCRIPTPATH}/.."

set -eux

for i in 0 1 2 3 4 5 6 7; do
	arguments=""
	if [ "$i" -eq 1 ]; then
		arguments="$i argument"
	else
		arguments="$i arguments"
	fi

	sed -e "s/__ARGUMENTS__/$arguments/g" \
		-e 's/__ADDENDUM__//g' \
		docs/syscall__TEMPLATE__.md >"docs/syscall${i}.md"

	sed -e "s/__ARGUMENTS__/$arguments/g" \
		-e "s/__ADDENDUM__/\\nLike the non \`_readonly\` version but you declare that syscall does not mutate any memory./g" \
		docs/syscall__TEMPLATE__.md >"docs/syscall${i}_readonly.md"
done

sed -e "s/__ARGUMENTS__/$arguments/g" \
	-e "s/__ADDENDUM__/\\nIt's assured that it will not return./g" \
	docs/syscall__TEMPLATE__.md >"docs/syscall1_noreturn.md"
