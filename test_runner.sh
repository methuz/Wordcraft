#!/bin/bash

echo "🧪 Running Wordcraft Test Suite"
echo "================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ $2${NC}"
    else
        echo -e "${RED}✗ $2${NC}"
    fi
}

# Clean previous builds
echo -e "${YELLOW}🧹 Cleaning previous builds...${NC}"
cargo clean

# Run unit tests
echo -e "${YELLOW}🔬 Running unit tests...${NC}"
cargo test --lib
unit_result=$?
print_status $unit_result "Unit tests"

# Run integration tests
echo -e "${YELLOW}🔧 Running integration tests...${NC}"
cargo test --test integration_tests
integration_result=$?
print_status $integration_result "Integration tests"

# Run all tests
echo -e "${YELLOW}🚀 Running all tests...${NC}"
cargo test
all_result=$?
print_status $all_result "All tests"

# Check for test coverage (if tarpaulin is available)
if command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${YELLOW}📊 Generating test coverage report...${NC}"
    cargo tarpaulin --out Html --output-dir coverage
    coverage_result=$?
    print_status $coverage_result "Coverage report generated"
    
    if [ $coverage_result -eq 0 ]; then
        echo -e "${GREEN}📈 Coverage report available at: coverage/tarpaulin-report.html${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-tarpaulin not installed. Skipping coverage report.${NC}"
    echo -e "${YELLOW}   Install with: cargo install cargo-tarpaulin${NC}"
fi

# Build the project
echo -e "${YELLOW}🏗️  Building project...${NC}"
cargo build
build_result=$?
print_status $build_result "Build"

# Run clippy for linting
echo -e "${YELLOW}🔍 Running clippy...${NC}"
cargo clippy -- -D warnings
clippy_result=$?
print_status $clippy_result "Clippy linting"

# Run format check
echo -e "${YELLOW}🎨 Checking code formatting...${NC}"
cargo fmt --check
fmt_result=$?
print_status $fmt_result "Code formatting"

# Summary
echo ""
echo "📋 Test Summary:"
echo "================"
print_status $unit_result "Unit tests"
print_status $integration_result "Integration tests"
print_status $all_result "All tests"
print_status $build_result "Build"
print_status $clippy_result "Clippy"
print_status $fmt_result "Formatting"

# Exit with error if any tests failed
if [ $unit_result -ne 0 ] || [ $integration_result -ne 0 ] || [ $all_result -ne 0 ] || [ $build_result -ne 0 ] || [ $clippy_result -ne 0 ] || [ $fmt_result -ne 0 ]; then
    echo -e "${RED}❌ Some checks failed!${NC}"
    exit 1
else
    echo -e "${GREEN}✅ All checks passed!${NC}"
    exit 0
fi