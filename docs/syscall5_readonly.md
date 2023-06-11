Make a raw system call with 5 arguments.
Like the non `_readonly` version but you declare that syscall does not mutate any memory.

# Safety

A system call is unsafe by definition.
It's the caller's responsibility to ensure safety.
