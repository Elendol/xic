cst_bin := ".tmp/container-structure-test-darwin-amd64"

[private]
@default:
    just --list

new-release step:
    cargo release {{ step }}

[macos]
install-cst:
    curl \
    --fail \
    --create-dirs \
    -L https://storage.googleapis.com/container-structure-test/latest/container-structure-test-darwin-amd64 \
    --output {{ cst_bin }}
    chmod +x {{ cst_bin }}

build-container:
    docker build --platform linux/amd64 --tag xic:latest .

test-container:
    {{ cst_bin }} test \
    --image xic:latest \
    --config tests/container.yaml
