FROM alpine:3.16 as build

WORKDIR /build

# update the Alpine package manager and install Rust and Cargo
RUN apk add --no-cache cargo musl-dev openssl-dev

# Copy over Cargo.toml files to the build directory
COPY template_generator/Cargo.toml template_generator/Cargo.toml 
COPY webserver/Cargo.toml webserver/Cargo.toml
COPY Cargo.toml Cargo.toml

# Build all dependencies first
RUN mkdir -p template_generator/src/bin \
    && mkdir -p webserver/src/bin \
    && echo "fn main() {}" > template_generator/src/bin/dummy.rs \
    && echo "fn main() {}" > webserver/src/bin/dummy.rs \
    && cargo build --release \
    && rm -rf template_generator/src/ \
    && rm -rf webserver/src/

# Then build the source
COPY template_generator/ template_generator/
COPY webserver/ webserver/
RUN cargo build --release

# Copy over templates and generate the static files
COPY content.json content.json
COPY templates templates
RUN cargo run --release --bin template_generator

FROM alpine:3.16 as server

WORKDIR /service

RUN apk add --no-cache libgcc

# Copy over the generated templates
COPY --from=build /build/templates/generated/ templates/generated/

# Copy over server executable from the build stage
COPY --from=build /build/target/release/webserver webserver

# Copy over static files to image
COPY docker-entrypoint.sh docker-entrypoint.sh
COPY www/ www/ 

EXPOSE 80 443
ENV HTTP_PORT 80
ENV HTTPS_PORT 443
ENV RUST_LOG info

RUN chmod +x docker-entrypoint.sh

CMD [ "./docker-entrypoint.sh" ]