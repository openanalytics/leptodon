{
  description = "Playwright testing environemnt";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # Versions <1.58 has a firefox regression for file uploads, holding of upgrading until nixpkgs-unstable updates to 1.59.
    # - https://github.com/microsoft/playwright/issues/39274
    nixpkgs-playwright.url = "github:NixOS/nixpkgs/145b67bd0bd4e075f981c1c2b81155d9e2982de2";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, nixpkgs-playwright, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgs-playwright = nixpkgs-playwright.legacyPackages.${system};
        browsers = (builtins.fromJSON (builtins.readFile "${pkgs-playwright.playwright-driver}/browsers.json")).browsers;
        chromium-rev = (builtins.head (builtins.filter (x: x.name == "chromium") browsers)).revision;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.pkg-config ];

          buildInputs = with pkgs; [
            jq
            nodejs
          ];

          PLAYWRIGHT_BROWSERS_PATH = "${pkgs-playwright.playwright.browsers}";
          PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = true;
          PLAYWRIGHT_NODEJS_PATH = "${pkgs.nodejs}/bin/node";
          PLAYWRIGHT_LAUNCH_OPTIONS_EXECUTABLE_PATH = "${pkgs-playwright.playwright.browsers}/chromium-${chromium-rev}/chrome-linux64/chrome";

          shellHook = ''
            playwrightNpmVersion="$(jq -r '.packages["node_modules/@playwright/test"].version' package-lock.json)"
            latestNixPlaywrightVersion="${pkgs.playwright.version}"
            echo "❄️ Playwright nix version: ${pkgs-playwright.playwright.version}"
            echo "📦 Playwright npm version: $playwrightNpmVersion"

            if [ "${pkgs-playwright.playwright.version}" != "$playwrightNpmVersion" ]; then
                echo "❌ Playwright versions in nix (in flake.nix) and npm (in package.json) are not the same! Please adapt the configuration."
            else
                echo "✅ Playwright versions in nix and npm are the same"

                if [ "$latestNixPlaywrightVersion" != $playwrightNpmVersion ]; then
                    echo "🔼 A newer playwright version is available: $latestNixPlaywrightVersion"
                    echo "   See: https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/development/web/playwright/driver.nix#L136"
                fi
            fi

            echo
            env | grep ^PLAYWRIGHT
          '';
        };
      }
    );
}
