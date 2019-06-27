#!/bin/bash
if ! command -v pod; then
    curl https://cocoapods-specs.circleci.com/fetch-cocoapods-repo-from-s3.sh | bash -s cf
fi

cd app
pod install
