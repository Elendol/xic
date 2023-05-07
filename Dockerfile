# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.69 as rust-build-env
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
WORKDIR /root
COPY . /root
ARG TARGETARCH
RUN cargo build --release

FROM --platform=$BUILDPLATFORM mono:6.12
WORKDIR /root
ARG TARGETARCH
RUN set -eux \
    && apt-get update \
    && apt-get install --no-install-recommends -y git  \
    && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch master https://github.com/compomics/ThermoRawFileParser
WORKDIR /root/ThermoRawFileParser
RUN msbuild -property:Configuration=Release
RUN set -eux  \
    && echo '#!/usr/bin/env bash\nmono /root/ThermoRawFileParser/bin/Release/ThermoRawFileParser.exe "$@"' > /usr/bin/ThermoRawFileParser \
    && chmod +x /usr/bin/ThermoRawFileParser \
    && ThermoRawFileParser xic --print_example > xic_input_example.json
COPY --from=rust-build-env /root/target/release/xic /usr/bin/xic
RUN chmod +x /usr/bin/xic
WORKDIR /workspace

LABEL org.opencontainers.image.source=https://github.com/Elendol/xic
LABEL org.opencontainers.image.licenses=MIT
LABEL org.opencontainers.image.authors=Elendol
