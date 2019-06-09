#!/bin/bash
PACKAGES_ROOT=packages/
COMMIT="refs/remotes/origin/master"
COMMAND=$1

CHANGED_PACKAGES=$(git diff --dirstat=files,0 $COMMIT | sed -r "s/[0-9.[:space:]]+% //g" | sed -r "s/(packages\/[[:alnum:]_-]+\/).*/\1/g" | uniq | grep $PACKAGES_ROOT)

if [[ $CHANGED_PACKAGES =~ "core" ]]; then
    CHANGED_PACKAGES=$(ls -1a -d $PACKAGES_ROOT/*/)
fi

if [[ -z $CHANGED_PACKAGES ]]; then
    echo Nothing to do.
    exit 0
fi

CWD=$(pwd)

while read -r line; do
    cd $line
    eval $COMMAND
    EXIT_CODE=$?
    if [[ $EXIT_CODE -ne 0 ]]; then
        exit $EXIT_CODE
    fi
    cd $CWD
done <<< "$CHANGED_PACKAGES"

