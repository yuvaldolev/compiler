#!/bin/bash

# If any command errors, stop the script.
set -e

# Set up directories.
ME=$(realpath "$0")
LOCATION=$(dirname "$ME")
PRJ_ROOT=$(dirname "$LOCATION")
SRC_ROOT="$PRJ_ROOT/src"
INTERMEDIATE_ROOT="$PRJ_ROOT/.build"

mkdir -p "$INTERMEDIATE_ROOT"

# Get the build mode.
BUILD_MODE="$1"
if [ -z "$BUILD_MODE" ]; then
	BUILD_MODE="-DBUILD_DEV"
fi

CC=clang
WARNINGS="-Wall -Werror"
COMPILER_FLAGS="-g -fPIC -fpermissive $BUILD_MODE"

# Execute
$CC $WARNINGS $COMPILER_FLAGS "$SRC_ROOT/build_compiler.c" -o "$INTERMEDIATE_ROOT/build_compiler"

pushd "$PRJ_ROOT" >/dev/null
"$INTERMEDIATE_ROOT/build_compiler"
popd >/dev/null

