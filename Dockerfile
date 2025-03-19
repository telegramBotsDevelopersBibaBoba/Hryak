FROM rust:1.83.0

WORKDIR /root

COPY . .

RUN cargo build --release

CMD ["/root/target/release/Hryak"]