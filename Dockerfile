# Runtime-only image: copy prebuilt musl binaries
FROM --platform=$TARGETPLATFORM alpine:3.19 AS runtime

ARG TARGETARCH

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libc6-compat

# Create non-root user
RUN addgroup -g 1000 app && adduser -u 1000 -G app -s /bin/sh -D app

# Copy binary for the current target architecture
COPY docker-bin/truenas-master-mcp-${TARGETARCH} /usr/local/bin/truenas-master-mcp

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/truenas-master-mcp

# Use non-root user
USER app

# Expose default HTTP/SSE port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/mcp || exit 1

# Environment variables with defaults
ENV TRUENAS_SERVER_URL=http://localhost
ENV TRUENAS_TIMEOUT=30
ENV TRUENAS_VERSION=scale
ENV TRUENAS_VERIFY_SSL=true

# Default command - run in SSE mode
CMD ["truenas-master-mcp", "--transport=sse", "--host=0.0.0.0", "--port=3000"]

# Alternative commands:
# stdio mode (for MCP clients):
# CMD ["truenas-master-mcp", "--transport=stdio"]
#
# HTTP mode:
# CMD ["truenas-master-mcp", "--transport=http", "--host=0.0.0.0", "--port=3000"]
