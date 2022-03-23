FROM alpine:latest

# update the Alpine package manager and install Rust and Cargo
RUN apk add --no-cache cargo musl-dev

# copy over source files to image
COPY . /website/

# change working directory to directory with source code
WORKDIR /website

RUN cargo build --release

# clean up build resources
RUN cp target/release/handler . && cargo clean
RUN apk del musl-dev

EXPOSE 3000

# ENTRYPOINT [ "/bin/sh" ]

CMD ["./handler"]
