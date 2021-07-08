FROM rust:latest as rust

LABEL org.opencontainers.image.source https://github.com/Trioxidation/Triox

WORKDIR /src
COPY . .
RUN cargo build --release 

FROM debian:buster
RUN useradd -ms /bin/bash -u 1001 triox
WORKDIR /home/triox/triox
COPY . .
COPY --from=rust /src/target/release/triox .
USER triox
CMD [ "/home/triox/triox/triox" ]
