#!/bin/bash
# OpenAPI Client Generation Script for TrueNAS
# This script fetches the TrueNAS OpenAPI spec and generates a Rust client

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPEC_DIR="${SCRIPT_DIR}/openapi-specs"
GENERATED_DIR="${SCRIPT_DIR}/src/generated"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}TrueNAS OpenAPI Client Generator${NC}"
echo "======================================"

# Check if TRUENAS_SERVER_URL is set
if [ -z "${TRUENAS_SERVER_URL}" ]; then
    echo -e "${YELLOW}TRUENAS_SERVER_URL not set. Using default: http://localhost${NC}"
    TRUENAS_SERVER_URL="http://localhost"
fi

if [ -z "${TRUENAS_API_KEY}" ]; then
    echo -e "${YELLOW}TRUENAS_API_KEY not set. Some API endpoints may not be accessible.${NC}"
fi

# Create directories
mkdir -p "${SPEC_DIR}"
mkdir -p "${GENERATED_DIR}"

# Step 1: Fetch the TrueNAS OpenAPI spec
echo ""
echo -e "${GREEN}[1/4] Fetching TrueNAS OpenAPI specification...${NC}"

SPEC_URL="${TRUENAS_SERVER_URL}/api/v2.0/docs/api.json"
SPEC_FILE="${SPEC_DIR}/truenas-api.json"

if command -v curl &> /dev/null; then
    if [ -n "${TRUENAS_API_KEY}" ]; then
        curl -s -H "Authorization: Bearer ${TRUENAS_API_KEY}" "${SPEC_URL}" -o "${SPEC_FILE}"
    else
        curl -s "${SPEC_URL}" -o "${SPEC_FILE}"
    fi
elif command -v wget &> /dev/null; then
    if [ -n "${TRUENAS_API_KEY}" ]; then
        wget -q -H "Authorization: Bearer ${TRUENAS_API_KEY}" "${SPEC_URL}" -O "${SPEC_FILE}"
    else
        wget -q "${SPEC_URL}" -O "${SPEC_FILE}"
    fi
else
    echo -e "${RED}Error: curl or wget is required${NC}"
    exit 1
fi

if [ ! -s "${SPEC_FILE}" ]; then
    echo -e "${RED}Error: Failed to fetch OpenAPI spec from ${SPEC_URL}${NC}"
    echo "Please check:"
    echo "  1. TrueNAS server is running and accessible"
    echo "  2. TRUENAS_SERVER_URL is correct"
    echo "  3. TRUENAS_API_KEY is valid (if required)"
    exit 1
fi

echo -e "  Spec saved to: ${SPEC_FILE}"
echo -e "  Spec size: $(du -h "${SPEC_FILE}" | cut -f1)"

# Step 2: Validate the spec
echo ""
echo -e "${GREEN}[2/4] Validating OpenAPI specification...${NC}"

if command -v jq &> /dev/null; then
    SPEC_VERSION=$(jq -r '.openapi // "unknown"' "${SPEC_FILE}")
    INFO_TITLE=$(jq -r '.info.title // "unknown"' "${SPEC_FILE}")
    INFO_VERSION=$(jq -r '.info.version // "unknown"' "${SPEC_FILE}")
    PATH_COUNT=$(jq -r '.paths | keys | length' "${SPEC_FILE}")

    echo -e "  OpenAPI Version: ${SPEC_VERSION}"
    echo -e "  Title: ${INFO_TITLE}"
    echo -e "  Version: ${INFO_VERSION}"
    echo -e "  API Paths: ${PATH_COUNT}"
else
    echo -e "${YELLOW}  jq not installed, skipping detailed validation${NC}"
fi

# Step 3: Generate Rust client using openapi-generator
echo ""
echo -e "${GREEN}[3/4] Generating Rust client...${NC}"

if ! command -v openapi-generator-cli &> /dev/null; then
    echo -e "${YELLOW}openapi-generator-cli not found.${NC}"
    echo "Installing via npm..."
    if command -v npm &> /dev/null; then
        npm install -g @openapitools/openapi-generator-cli
    elif command -v cargo &> /dev/null; then
        # Try installing via cargo if available
        echo "Note: npm is required for openapi-generator-cli"
        echo "Please install: npm install -g @openapitools/openapi-generator-cli"
    fi
fi

if command -v openapi-generator-cli &> /dev/null; then
    GENERATED_TMP_DIR=$(mktemp -d)
    echo "  Using openapi-generator-cli..."

    openapi-generator-cli generate \
        -i "${SPEC_FILE}" \
        -g rust \
        -o "${GENERATED_TMP_DIR}/rust-client" \
        --additional-properties=useSingleRequestParameter=true,feature=async,library=reqwest \
        2>/dev/null || true

    if [ -d "${GENERATED_TMP_DIR}/rust-client/src" ]; then
        # Copy generated files
        cp -r "${GENERATED_TMP_DIR}/rust-client/src/"* "${GENERATED_DIR}/" 2>/dev/null || true
        cp -r "${GENERATED_TMP_DIR}/rust-client/Cargo.toml" "${GENERATED_DIR}/" 2>/dev/null || true

        # Copy models
        mkdir -p "${GENERATED_DIR}/models"
        cp -r "${GENERATED_TMP_DIR}/rust-client/src/models/"* "${GENERATED_DIR}/models/" 2>/dev/null || true

        echo -e "  Generated files copied to: ${GENERATED_DIR}"
    else
        echo -e "${YELLOW}  Warning: openapi-generator-cli may have failed${NC}"
    fi

    rm -rf "${GENERATED_TMP_DIR}"
else
    echo -e "${YELLOW}  openapi-generator-cli not available.${NC}"
    echo "  Generating minimal types from spec..."

    # Generate minimal types using jq
    if command -v jq &> /dev/null; then
        python3 "${SCRIPT_DIR}/scripts/generate_types.py" \
            "${SPEC_FILE}" \
            "${GENERATED_DIR}/models.rs" \
            2>/dev/null || echo "  Note: Run 'pip install jinja2' for better type generation"
    fi
fi

# Step 4: Summary
echo ""
echo -e "${GREEN}[4/4] Summary${NC}"
echo "============="

if [ -d "${GENERATED_DIR}" ]; then
    FILE_COUNT=$(find "${GENERATED_DIR}" -name "*.rs" | wc -l)
    echo -e "  Generated ${FILE_COUNT} Rust files"
    echo ""
    echo "Next steps:"
    echo "  1. Review generated code in: ${GENERATED_DIR}"
    echo "  2. Update Cargo.toml with any additional dependencies"
    echo "  3. Integrate with existing MCP server in src/tools.rs"
    echo "  4. Run: cargo check"
else
    echo -e "${YELLOW}  No files generated. Please install openapi-generator-cli.${NC}"
fi

echo ""
echo "To generate client from a spec file directly:"
echo "  openapi-generator-cli generate -i <spec-file> -g rust -o /tmp/client"
