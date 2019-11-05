#!/bin/sh

PKG_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
DIST_NAME="pastelogue_${PKG_VERSION}_MacOS"
DIST_PATH="dist/${DIST_NAME}"

mkdir -p $DIST_PATH

cp ./target/release/pastelogue $DIST_PATH
cp ./build/exiv2/* $DIST_PATH

cd dist
tar -czvf "${DIST_NAME}.tar.gz" $DIST_NAME
cd ..

rm -rf $DIST_PATH
