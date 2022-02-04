sh build-scripts/prepare-workspace.sh
cargo build --release
cp target/release/handler .
func start --verbose
sh build-scripts/clean-workspace.sh