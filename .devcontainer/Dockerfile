FROM ghcr.io/swelio/dev-images/base/rust:latest

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    file \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    unzip \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,sharing=locked,target=/opt/cargo \
    --mount=type=bind,source=.cargo/installer.config.toml,target=/opt/cargo/config.toml \
    CARGO_HOME=/opt/cargo cargo install \
    create-tauri-app \
    jaq \
    tauri-cli \
    && CARGO_HOME=/opt/cargo cargo install --no-default-features --features=rustls trunk \
    && rm /usr/local/cargo/.crates.toml /usr/local/cargo/.crates2.json \
    && chmod --recursive 0755 /usr/local/cargo

RUN rustup target add wasm32-unknown-unknown

ADD https://github.com/oven-sh/bun/releases/latest/download/bun-linux-x64.zip /opt/bun-linux-x64.zip
RUN unzip /opt/bun-linux-x64.zip -d /opt \
    && mv /opt/bun-linux-x64/bun /usr/local/bin/bun \
    && chmod 0755 /usr/local/bin/bun \
    && rm -r /opt/bun-linux-x64.zip /opt/bun-linux-x64
