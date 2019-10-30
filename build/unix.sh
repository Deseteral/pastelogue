#!/bin/sh

cd build

if [ "$(uname)" == "Darwin" ]; then
    curl -o exiv2.tar.gz https://www.exiv2.org/builds/exiv2-0.27.2-Darwin.tar.gz
else
    curl -o exiv2.tar.gz https://www.exiv2.org/builds/exiv2-0.27.2-Linux64.tar.gz
fi

mkdir -p exiv2
tar xzf exiv2.tar.gz -C exiv2
mv exiv2/exiv2-0.27.2-Darwin/bin/exiv2json exiv2/exiv2json
rm -rf exiv2/exiv2-0.27.2-Darwin exiv2.tar.gz
