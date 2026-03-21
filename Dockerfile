# Build stage
FROM rust:1.85-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf perl make
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

# Runtime stage
FROM alpine:3.19
RUN apk add --no-cache ca-certificates libc6-compat
RUN addgroup -g 1000 app && adduser -u 1000 -G app -s /bin/sh -D app
COPY --from=builder /app/target/release/truenas-master-mcp /usr/local/bin/truenas-master-mcp
RUN chmod +x /usr/local/bin/truenas-master-mcp
USER app
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/sse || exit 1

ENV TRUENAS_SERVER_URL=http://localhost
ENV TRUENAS_TIMEOUT=30
ENV TRUENAS_VERSION=scale
ENV TRUENAS_VERIFY_SSL=false

CMD ["truenas-master-mcp", "--transport=sse", "--host=0.0.0.0", "--port=3000"]
