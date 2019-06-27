#!/bin/bash
if [[ "$OSTYPE" != "darwin"* ]]; then
    exit 0
fi

curl https://cocoapods-specs.circleci.com/fetch-cocoapods-repo-from-s3.sh | bash -s cf
cd app
pod install
