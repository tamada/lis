#! /bin/sh

UPLOAD_URL=$1
TAG=$2

for i in dist/*-${TAG}-*.tar.gz
do
    echo "Uploading $i to $UPLOAD_URL"
    gh release upload $TAG $i
done
