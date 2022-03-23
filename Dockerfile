FROM alpine:latest

# update the Alpine package manager and install Rust and Cargo
RUN apk add --update --no-cache rust cargo musl-dev

# copy over source files to image
COPY . /website/

# change working directory to directory with source code
WORKDIR /website

EXPOSE 3000

RUN cargo build --release

RUN cp target/release/handler . && rm -rf target

RUN apk del rust cargo musl-dev

ENTRYPOINT [ "/bin/sh" ]

# CMD ["./handler"]
