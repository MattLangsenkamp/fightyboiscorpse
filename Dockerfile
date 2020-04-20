FROM rust:1.31

COPY . .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

run rustup default nightly
RUN cargo install --path .

CMD cargo run