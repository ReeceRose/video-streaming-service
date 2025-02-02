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
COPY services/notification/Cargo.toml services/notification/
COPY . .

RUN cargo build -p video_on_demand_notification_service
RUN cargo install cargo-watch

EXPOSE 8082

CMD ["cargo", "watch", "-x", "run -p video_on_demand_notification_service"]
