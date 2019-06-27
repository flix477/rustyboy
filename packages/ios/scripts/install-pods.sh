#!/bin/bash
if ! command -v pod; then
    sudo gem install cocoapods
fi

cd app
pod install
