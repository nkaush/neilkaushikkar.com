# Prepare workspace
sh azure-scripts/prepare-workspace.sh

# Build
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/handler .

# Load login details from file
login_details=()
while IFS= read -r line || [[ "$line" ]]; do
  login_details+=("$line")
done < azure-scripts/login-details.txt

# Deploy
az login -u ${login_details[1]} -p ${login_details[2]}
func azure functionapp publish ${login_details[0]}

# Clean up working directory
sh azure-scripts/clean-workspace.sh