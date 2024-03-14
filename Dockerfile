FROM rust:bookworm as builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM archlinux

RUN pacman-key --init && \
	pacman-key --populate archlinux

# Install dependencies for yt-dlp and ffmpeg
RUN pacman -Sy --noconfirm archlinux-keyring && \
	pacman -Syu --noconfirm && \
    pacman -S --noconfirm \
    ca-certificates \
    ffmpeg \
    yt-dlp \
    aria2 \
    python3 \
    python-mutagen

COPY --from=builder /app/target/release/hoard /hoard
COPY ./entrypoint.sh /entrypoint.sh

WORKDIR /

CMD ["/bin/bash", "/entrypoint.sh"]
