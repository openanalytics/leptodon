#!/bin/sh
set +e;
npm install;

# This script supports nixos with devenv or ubuntu:24 due to playwright.
if command -v devenv >/dev/null 2>&1
then
    CI=1 devenv shell playwright test;
else
    npx playwright install-deps;
    npx playwright install;
    CI=1 npx playwright test;
fi
