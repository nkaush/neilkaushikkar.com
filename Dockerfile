FROM alpine:3.16 as build

WORKDIR /build

# update the Alpine package manager and install Rust and Cargo
RUN apk add --no-cache cargo musl-dev openssl-dev

COPY Cargo.toml Cargo.toml

# Build all dependencies first
RUN mkdir -p src/bin \
    && echo "fn main() {}" > src/bin/dummy.rs \
    && cargo build --release --bin dummy \
    && rm -rf src/

# Then build the source
COPY src/ src/
RUN cargo build --release

FROM alpine:3.16 as template_generator

WORKDIR /build

RUN apk add --no-cache libgcc

# Copy over template generator executable from the build stage
COPY --from=build /build/target/release/template_generator template_generator

COPY content.json content.json
COPY templates templates

RUN ./template_generator

FROM alpine:3.16 as server

WORKDIR /service

RUN apk add --no-cache libgcc

# Copy over the generated templates
COPY --from=template_generator /build/templates/generated/ templates/generated/

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