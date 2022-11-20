exec 1>&2
pushd livesplit-core
git reset --hard
git apply --3way ../livesplit_core.patch
cargo build --release -p staticlib --features auto-splitting
popd
cp livesplit-core/target/release/liblivesplit_core.a $3
