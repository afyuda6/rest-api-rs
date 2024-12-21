FROM rust:1.83-slim

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /rest-api-rs

COPY . /rest-api-rs

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/rest-api-rs"]