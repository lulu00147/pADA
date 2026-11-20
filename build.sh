#!/bin/sh
set -e
PROJECT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$PROJECT_DIR"
SOURCE_DIR="${DUCKDB_SOURCE_DIR:-$PROJECT_DIR/duckdb-1.5.4}"
if [ ! -d "$SOURCE_DIR" ] && [ -d "$PROJECT_DIR/../duckdb-1.5.4" ]; then
  SOURCE_DIR="$PROJECT_DIR/../duckdb-1.5.4"
fi
BUILD_DIR="$PROJECT_DIR/build-gcc"
CACHE_DIR="$PROJECT_DIR/duckdb-cache/x86_64-unknown-linux-gnu-gcc"
GIT_DESCRIBE="v1.5.4-0-gf4c72b5"
command -v cargo >/dev/null || { echo "[error] cargo/rustup not found"; exit 1; }
for c in cmake ninja gcc g++ make; do
  command -v "$c" >/dev/null || { echo "[error] missing: $c (sudo apt install -y build-essential cmake ninja-build)"; exit 1; }
done
if [ ! -f "$SOURCE_DIR/CMakeLists.txt" ]; then
  echo "[error] DuckDB source not found (set DUCKDB_SOURCE_DIR): $SOURCE_DIR"
  exit 1
fi
if [ -n "$1" ] && [ "$1" = "rebuild" ]; then
  rm -rf "$BUILD_DIR"
fi
if [ -f "$CACHE_DIR/.built-gcc-$GIT_DESCRIBE" ] && [ -z "$1" ]; then
  echo "[cache] DuckDB gcc libraries already built"
else
  echo "[duckdb] configuring ($GIT_DESCRIBE)"
  cmake -S "$SOURCE_DIR" -B "$BUILD_DIR" -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DOVERRIDE_GIT_DESCRIBE="$GIT_DESCRIBE" \
    -DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF \
    -DBUILD_EXTENSIONS="json;parquet"
  echo "[duckdb] building static libraries"
  ninja -C "$BUILD_DIR" duckdb_static duckdb_generated_extension_loader \
        json_extension parquet_extension core_functions_extension
  mkdir -p "$CACHE_DIR"
  cp "$BUILD_DIR/src/libduckdb_static.a" "$CACHE_DIR/"
  cp "$BUILD_DIR/extension/libduckdb_generated_extension_loader.a" "$CACHE_DIR/"
  cp "$BUILD_DIR/extension/json/libjson_extension.a" "$CACHE_DIR/"
  cp "$BUILD_DIR/extension/parquet/libparquet_extension.a" "$CACHE_DIR/"
  cp "$BUILD_DIR/extension/core_functions/libcore_functions_extension.a" "$CACHE_DIR/"
  cp "$SOURCE_DIR/src/include/duckdb.h" "$CACHE_DIR/"
  : > "$CACHE_DIR/.built-gcc-$GIT_DESCRIBE"
  echo "[duckdb] cached to $CACHE_DIR"
fi
export DUCKDB_LIB_DIR="$CACHE_DIR"
export DUCKDB_INCLUDE_DIR="$CACHE_DIR"
export DUCKDB_STATIC=1
export DUCKDB_CXX_LIB=stdc++
echo "[rust] building pADA"
cargo build --release --target x86_64-unknown-linux-gnu --features http --bins
echo "[ok] $PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/pADA"
