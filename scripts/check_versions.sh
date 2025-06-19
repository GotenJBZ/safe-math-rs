#!/bin/bash

# Get the version from the root Cargo.toml
ROOT_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | cut -d '"' -f 2)

# Get the version from safe-math-macros/Cargo.toml
MACROS_VERSION=$(grep '^version = ' safe-math-macros/Cargo.toml | head -n 1 | cut -d '"' -f 2)

# Check if versions match
if [ "$ROOT_VERSION" != "$MACROS_VERSION" ]; then
    echo "Version mismatch detected!"
    echo "Root Cargo.toml version: $ROOT_VERSION"
    echo "safe-math-macros/Cargo.toml version: $MACROS_VERSION"
    echo "Please ensure both versions match."
    exit 1
fi

# Also check the dependency version in root Cargo.toml matches the macros version
DEP_VERSION=$(grep 'safe-math-macros.*version.*=' Cargo.toml | head -n 1 | cut -d '"' -f 2)
if [ "$ROOT_VERSION" != "$DEP_VERSION" ]; then
    echo "Dependency version mismatch detected!"
    echo "Root package version: $ROOT_VERSION"
    echo "safe-math-macros dependency version: $DEP_VERSION"
    echo "Please ensure the dependency version matches the package version."
    exit 1
fi

exit 0 