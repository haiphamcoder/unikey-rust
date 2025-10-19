#!/bin/bash
# Build script for UniKey Rust

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
BUILD_TYPE="debug"
TARGET=""
FEATURES=""
PACKAGE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --package)
            PACKAGE="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --release        Build in release mode"
            echo "  --target TARGET  Build for specific target"
            echo "  --features FEAT  Enable specific features"
            echo "  --package PKG    Build specific package only"
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${GREEN}Building UniKey Rust...${NC}"

# Build command
BUILD_CMD="cargo build"

if [[ "$BUILD_TYPE" == "release" ]]; then
    BUILD_CMD="$BUILD_CMD --release"
fi

if [[ -n "$TARGET" ]]; then
    BUILD_CMD="$BUILD_CMD --target $TARGET"
fi

if [[ -n "$FEATURES" ]]; then
    BUILD_CMD="$BUILD_CMD --features $FEATURES"
fi

if [[ -n "$PACKAGE" ]]; then
    BUILD_CMD="$BUILD_CMD -p $PACKAGE"
fi

echo -e "${YELLOW}Running: $BUILD_CMD${NC}"

# Run the build
eval $BUILD_CMD

if [[ $? -eq 0 ]]; then
    echo -e "${GREEN}Build successful!${NC}"
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
