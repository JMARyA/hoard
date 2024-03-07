FROM rust:bookworm as builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM debian:bookworm

# Install dependencies for yt-dlp and ffmpeg
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    ffmpeg \
    yt-dlp \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hoard /hoard

WORKDIR /

CMD ["/hoard"]