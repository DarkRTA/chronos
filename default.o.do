. ./cflags.rc
mkdir -p "$(dirname "$1")"
file="${2#obj/}.c"
redo-ifchange "$file"
$CC -c -o "$3" "$file" $CFLAGS
read DEPS < "${3%.tmp}".d
redo-ifchange ${DEPS#*:}
