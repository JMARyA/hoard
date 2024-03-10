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
    python3-pip

RUN pip3 install --break-system-packages mutagen

COPY --from=builder /app/target/release/hoard /hoard

WORKDIR /

CMD ["/hoard"]
