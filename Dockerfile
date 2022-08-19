FROM alpine:latest as build

WORKDIR /build

# update the Alpine package manager and install Rust and Cargo
RUN apk add --no-cache cargo musl-dev openssl-dev

COPY Cargo.toml Cargo.toml

# Build all dependencies first
RUN mkdir src/ \
    && touch src/lib.rs \
    && cargo build --release \
    && rm -rf src/

# Then build the source
COPY src/ src/
RUN cargo build --release

FROM alpine:latest as server

WORKDIR /service

RUN apk add --no-cache libgcc

# Copy over executable from the build stage
COPY --from=build /build/target/release/handler handler

# Copy over static files to image
COPY docker-entrypoint.sh docker-entrypoint.sh
COPY content.json content.json
COPY templates/ templates/
COPY www/ www/ 

EXPOSE 80 443
ENV HTTP_PORT 80
ENV HTTPS_PORT 443
ENV RUST_LOG info

RUN chmod +x docker-entrypoint.sh

CMD ["./docker-entrypoint.sh"]