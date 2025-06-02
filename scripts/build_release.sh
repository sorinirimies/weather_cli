#!/bin/bash
set -e

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Weather Man Optimized Build Script ===${NC}"

# Determine target based on current system or argument
if [ -n "$1" ]; then
  TARGET="$1"
else
  # Auto-detect the current system
  case "$(uname -s)" in
    Linux*)     
      if [ "$(uname -m)" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
      else
        TARGET="x86_64-unknown-linux-gnu"
      fi
      ;;
    Darwin*)    
      if [ "$(uname -m)" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
      else
        TARGET="x86_64-apple-darwin"
      fi
      ;;
    CYGWIN*|MINGW*|MSYS*)
      TARGET="x86_64-pc-windows-msvc"
      ;;
    *)
      echo -e "${RED}Unsupported operating system. Please specify target manually.${NC}"
      exit 1
      ;;
  esac
fi

echo -e "${YELLOW}Building optimized release for target:${NC} $TARGET"

# Check if the target is installed
if ! rustup target list --installed | grep -q "$TARGET"; then
  echo -e "${YELLOW}Target $TARGET not installed. Installing...${NC}"
  rustup target add "$TARGET"
fi

# Check cargo-bloat is installed for binary size analysis
if ! command -v cargo-bloat &> /dev/null; then
  echo -e "${YELLOW}cargo-bloat not found. Installing...${NC}"
  cargo install cargo-bloat
fi

# Build with optimizations
echo -e "${BLUE}Building optimized binary...${NC}"
RUSTFLAGS="-C target-cpu=native" cargo build --release --target "$TARGET"

# Show binary size
BINARY_PATH="target/$TARGET/release/weather_man"
if [[ "$TARGET" == *"windows"* ]]; then
  BINARY_PATH="$BINARY_PATH.exe"
fi

echo -e "${GREEN}Build complete!${NC}"
echo -e "${YELLOW}Binary size:${NC} $(du -h "$BINARY_PATH" | cut -f1)"

# Run bloat analysis
echo -e "${BLUE}Running size analysis...${NC}"
cargo bloat --release --target "$TARGET"

# Create distributable package
PACKAGE_DIR="dist"
mkdir -p "$PACKAGE_DIR"

VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)
PACKAGE_NAME="weather_man-v$VERSION-$TARGET"

echo -e "${BLUE}Creating distributable package: $PACKAGE_NAME${NC}"

if [[ "$TARGET" == *"windows"* ]]; then
  if command -v zip &> /dev/null; then
    zip -j "$PACKAGE_DIR/$PACKAGE_NAME.zip" "$BINARY_PATH" README.md LICENSE
    echo -e "${GREEN}Package created:${NC} $PACKAGE_DIR/$PACKAGE_NAME.zip"
  else
    echo -e "${RED}Warning: zip not installed. Could not create Windows package.${NC}"
  fi
else
  tar -czf "$PACKAGE_DIR/$PACKAGE_NAME.tar.gz" -C "$(dirname "$BINARY_PATH")" "$(basename "$BINARY_PATH")" -C "$(pwd)" README.md LICENSE
  echo -e "${GREEN}Package created:${NC} $PACKAGE_DIR/$PACKAGE_NAME.tar.gz"
fi

echo -e "${GREEN}Done!${NC}"