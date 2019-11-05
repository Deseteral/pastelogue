#!/bin/sh

EXIV2_BUILD_NAME="exiv2-0.27.2-Darwin"
EXIV2_LIB_NAME="libexiv2.0.27.2.dylib"
EXIV2_LIB_RELEASE_NAME="libexiv2.27.dylib"

mkdir -p release/exiv2
cd release
curl -o exiv2.tar.gz "https://www.exiv2.org/builds/$EXIV2_BUILD_NAME.tar.gz"
tar xzf exiv2.tar.gz -C exiv2

mv "exiv2/$EXIV2_BUILD_NAME/bin/exiv2json" exiv2/exiv2json
mv "exiv2/$EXIV2_BUILD_NAME/lib/$EXIV2_LIB_NAME" "exiv2/$EXIV2_LIB_RELEASE_NAME"

rm -rf "exiv2/$EXIV2_BUILD_NAME" exiv2.tar.gz

cd ..
