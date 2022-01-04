# Ubuntu dockerfile runner
FROM rust:1.31
WORKDIR D:\\Ascendance\
COPY . .
RUN cargo install --path .
CMD ["ascendance-game"]

FROM ubuntu:latest
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ascendance-game /usr/local/bin/ascendance-game
CMD ["ascendance-game"]
