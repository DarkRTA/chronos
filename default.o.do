. ./cflags.rc
mkdir -p "$(dirname "$1")"
file="${2#obj/}.c"
redo-ifchange "$file"
# shellcheck disable=SC2086
$CC -c -o "$3" "$file" $CFLAGS
# shellcheck disable=SC2162
read DEPS < "${3%.tmp}".d
# shellcheck disable=SC2086
redo-ifchange ${DEPS#*:}
