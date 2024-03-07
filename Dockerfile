FROM rust:buster as builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM archlinux

# Install dependencies for yt-dlp and ffmpeg
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm ca-certificates ffmpeg yt-dlp && \
    rm -rf /var/cache/pacman/pkg/*

COPY --from=builder /app/target/release/hoard /hoard

WORKDIR /

CMD ["/hoard"]