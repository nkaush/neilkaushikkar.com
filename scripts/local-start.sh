sh scripts/prepare-workspace.sh
cargo build --release
cp target/release/handler .
func start --verbose
sh scripts/clean-workspace.sh