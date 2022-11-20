source ./cflags.rc
OBJ=$(
	find src deps -type f -name '*.c' \
		| sed \
			-e 's@^@obj/@g' \
			-e 's@\.c$@.o@g'
)

redo-ifchange livesplit-core/liblivesplit_core.a
redo-ifchange livesplit-core/livesplit_core.h
redo-ifchange $OBJ

$CC -o $3 $OBJ $LDFLAGS
