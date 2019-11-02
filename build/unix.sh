#!/bin/sh

cd build

EXIV2_BUILD_NAME=''
if [ "$(uname)" == "Darwin" ]; then
    EXIV2_BUILD_NAME="exiv2-0.27.2-Darwin"
else
    EXIV2_BUILD_NAME="exiv2-0.27.2-Linux64"
fi

curl -o exiv2.tar.gz "https://www.exiv2.org/builds/$EXIV2_BUILD_NAME.tar.gz"
mkdir -p exiv2
tar xzf exiv2.tar.gz -C exiv2

mv "exiv2/$EXIV2_BUILD_NAME/bin/exiv2json" exiv2/exiv2json
mv "exiv2/$EXIV2_BUILD_NAME/lib/libexiv2.0.27.2.dylib" exiv2/libexiv2.0.27.2.dylib

rm -rf "exiv2/$EXIV2_BUILD_NAME" exiv2.tar.gz
