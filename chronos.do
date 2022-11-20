. ./cflags.rc

# XXX: this will break if there are spaces in the filename
# perhaps we should use a compiler wrapper script and set IFS
OBJ=$(
	find src deps -type f -name '*.c' \
		| sed \
			-e 's@^@obj/@g' \
			-e 's@\.c$@.o@g'
)

redo-ifchange livesplit-core/liblivesplit_core.a
redo-ifchange livesplit-core/livesplit_core.h

# shellcheck disable=SC2086
redo-ifchange $OBJ

# shellcheck disable=SC2086
$CC -o "$3" $OBJ $LDFLAGS
