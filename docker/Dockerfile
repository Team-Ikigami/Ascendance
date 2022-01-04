# Ubuntu dockerfile runner
FROM rust:1.40 as builder
WORKDIR D:\\Ascendance\
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
RUN apt-get update && apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev libxcb1-dev libxkbcommon-dev libasound2-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ascendance-game /usr/local/bin/ascendance-game
CMD ["ascendance-game"]
