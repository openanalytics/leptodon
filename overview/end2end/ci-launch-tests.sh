#!/bin/sh
set +e;

# This script supports a system with nix or ubuntu:24 due to playwright.
if command -v nix >/dev/null 2>&1
then
    CI=1 nix develop -c bash -c "
        npm i &&
        npx playwright test
    ";
else
    npm i;
    npx playwright install-deps;
    npx playwright install;
    CI=1 npx playwright test;
fi
