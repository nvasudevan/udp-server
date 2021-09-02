FROM rust:1.54-slim

WORKDIR /var/tmp
COPY . udp-server
RUN cd udp-server && cargo build --release && cp ./target/release/udp-server /opt/udp-server
RUN rm -rf udp-server
EXPOSE 6001/udp

CMD ["/opt/udp-server"]
