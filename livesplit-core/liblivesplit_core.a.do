exec 1>&2
(
	cd livesplit-core || exit 1
	git reset --hard
	git apply --3way ../livesplit_core.patch
	cargo build --release -p staticlib --features auto-splitting
)
cp livesplit-core/target/release/liblivesplit_core.a "$3"
