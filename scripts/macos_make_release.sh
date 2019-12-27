#!/bin/sh

PKG_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
RELEASE_NAME="pastelogue_${PKG_VERSION}_MacOS"
RELEASE_PATH="release/${RELEASE_NAME}"

mkdir -p $RELEASE_PATH

cp ./target/release/pastelogue $RELEASE_PATH
cp ./release/exiv2/* $RELEASE_PATH

cd release
tar -czvf "${RELEASE_NAME}.tar.gz" $RELEASE_NAME
cd ..

rm -rf $RELEASE_PATH
