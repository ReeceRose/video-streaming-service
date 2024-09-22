FROM rust:1.81

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    clang \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY services/video/Cargo.toml services/video/
COPY . .

RUN cargo build -p video
RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run -p video_on_demand_transcoding_service"]
