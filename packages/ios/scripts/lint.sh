#!/bin/bash
if [[ "$OSTYPE" != "darwin"* ]]; then
    exit 0
fi

cd app
./Pods/SwiftLint/swiftlint --strict
