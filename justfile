set shell := ["bash", "-uc"]
cst_bin := ".tmp/container-structure-test"

trfp_xic_json_example := "/root/ThermoRawFileParser/xic_input_example.json"
trfp_xic_raw_example := "/root/ThermoRawFileParser/ThermoRawFileParserTest/Data/small.RAW"

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
    docker build --tag xic:latest .

test-container:
    {{ cst_bin }} test \
    --image xic:latest \
    --config tests/container.yaml

get-test-data:
    docker run --rm \
    --volume ${PWD}/tests:/workspace \
    xic:latest \
    cp {{trfp_xic_json_example}} /workspace/xic_input.json
    docker run --rm \
    --volume ${PWD}/tests:/workspace \
    xic:latest \
    ThermoRawFileParser xic \
    --input={{trfp_xic_raw_example}} \
    --json={{trfp_xic_json_example}} \
    --output_file=/workspace/xic_output.json
    docker run --rm \
    --volume ${PWD}/tests:/workspace \
    xic:latest \
    ThermoRawFileParser xic \
    --input={{trfp_xic_raw_example}} \
    --json={{trfp_xic_json_example}} \
    --base64 \
    --output_file=/workspace/xic_output_base64.json

test:
    cargo run -- --xic-json tests/xic_input.json --xic-output tests/xic_output.json

test-base64:
    cargo run -- --xic-json tests/xic_input.json --xic-output tests/xic_output_base64.json
