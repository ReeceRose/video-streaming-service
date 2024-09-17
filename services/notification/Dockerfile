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
COPY services/notification/Cargo.toml services/notification/
COPY . .

RUN cargo build -p notification
RUN cargo install cargo-watch

EXPOSE 8082

CMD ["cargo", "watch", "-x", "run -p notification"]
