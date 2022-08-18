FROM alpine:latest as build

WORKDIR /build

# update the Alpine package manager and install Rust and Cargo
RUN apk add --no-cache cargo musl-dev

# copy over source code to build stage
COPY src/ src/
COPY Cargo.toml Cargo.toml

# Build it!
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

EXPOSE 3000
ENV PORT 3000

RUN chmod +x docker-entrypoint.sh

CMD ["./docker-entrypoint.sh"]