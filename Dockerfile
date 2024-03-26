FROM rust AS builder
COPY . .
RUN cargo build --release

FROM archlinux
RUN pacman -Sy --noconfirm git && rm -rf /var/cache/pacman/pkg/*
COPY --from=builder ./target/release/fml ./target/release/fml
COPY --from=builder Rocket.toml Rocket.toml
ENV RUST_BACKTRACE 1
CMD ["/target/release/fml"]
