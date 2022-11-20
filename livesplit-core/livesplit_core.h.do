exec 1>&2
pushd livesplit-core/capi/bind_gen
cargo run
popd
cp livesplit-core/capi/bindings/livesplit_core.h $3
