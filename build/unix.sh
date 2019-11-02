#!/bin/sh

EXIV2_BUILD_NAME=""
EXIV2_LIB_NAME=""
EXIV2_LIB_DIST_NAME=""

if [ "$(uname)" == "Darwin" ]; then
    EXIV2_BUILD_NAME="exiv2-0.27.2-Darwin"
    EXIV2_LIB_NAME="libexiv2.0.27.2.dylib"
    EXIV2_LIB_DIST_NAME="libexiv2.27.dylib"
else
    EXIV2_BUILD_NAME="exiv2-0.27.2-Linux64"
    EXIV2_LIB_NAME="libexiv2.so.0.27.2"
    EXIV2_LIB_DIST_NAME="libexiv2.so.27"
fi


cd build
curl -o exiv2.tar.gz "https://www.exiv2.org/builds/$EXIV2_BUILD_NAME.tar.gz"
mkdir -p exiv2
tar xzf exiv2.tar.gz -C exiv2

mv "exiv2/$EXIV2_BUILD_NAME/bin/exiv2json" exiv2/exiv2json
mv "exiv2/$EXIV2_BUILD_NAME/lib/$EXIV2_LIB_NAME" "exiv2/$EXIV2_LIB_DIST_NAME"

rm -rf "exiv2/$EXIV2_BUILD_NAME" exiv2.tar.gz
