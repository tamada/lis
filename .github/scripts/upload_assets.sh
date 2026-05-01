#! /bin/sh

UPLOAD_URL=$1
TAG=$2

for i in dist/*.tar.gz
do
    echo "Uploading $i to $UPLOAD_URL"
    gh release upload v$TAG $i
done
