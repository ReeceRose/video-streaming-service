FROM rust:1.83

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    clang \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY services/transcoding/Cargo.toml services/transcoding/
COPY . .

RUN cargo build -p video_on_demand_transcoding_service
RUN cargo install cargo-watch

EXPOSE 8081

CMD ["cargo", "watch", "-x", "run -p video_on_demand_transcoding_service"]
