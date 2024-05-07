#!/bin/bash

CMD_DIR=$(pwd)
BUILD_ARTIFACTS_DIR="$CMD_DIR/target/x86_64-unknown-linux-musl/debug"
DOCKER_BIN="docker/bin"
DOCKER_BIN_DIR="$CMD_DIR/$DOCKER_BIN"
TARGET_BIN_FILES=("node")

mkdir -p "$DOCKER_BIN_DIR"

for file in "${TARGET_BIN_FILES[@]}"; do
    SOURCE_FILE="$BUILD_ARTIFACTS_DIR/$file"
    if [ -f "$SOURCE_FILE" ]; then
        cp "$SOURCE_FILE" "$DOCKER_BIN_DIR"
        echo "Copied $file to $DOCKER_BIN_DIR"
    else
        echo "Warning: File $file does not exist in $BUILD_DIR"
    fi
done

for file in "${TARGET_BIN_FILES[@]}"; do
    SOURCE_FILE="$DOCKER_BIN/$file"
    if [ -f "$SOURCE_FILE" ]; then
        docker build \
        	--build-arg="ARTIFACT_BIN=$DOCKER_BIN" \
        	--build-arg="ARTIFACT=$file" \
        	--file docker/Dockerfile \
        	--tag "$file:latest" .
        echo "Docker image built for $file"

    else
        echo "Warning: File $SOURCE_FILE does not exist for Docker build"
    fi
done

rm -rf "$DOCKER_BIN_DIR"
echo "Deleted $DOCKER_BIN_DIR with all its contents."