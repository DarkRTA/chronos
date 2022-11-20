exec 1>&2
(
	cd livesplit-core/capi/bind_gen || exit 1
	cargo run
)
cp livesplit-core/capi/bindings/livesplit_core.h "$3"
