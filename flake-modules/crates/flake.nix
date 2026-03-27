# Copyright (c) 2022 Ivan Petkov
{
  description = "Build a cargo workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;

        craneLib = crane.mkLib pkgs;

        # Build custom cargo-leptos
        cargo-leptos = craneLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "ToxicMushroom";
            repo = "cargo-leptos";
            rev = "56a05748dfc27f621a0c65faea990d5fa1f13b48";
            hash = "sha256-NU39W3KfNV8bsLLX0RkOHjKuV/K9iD0CTEqQIofLpi4=";
          };
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          OPENSSL_NO_VENDOR = 1;
          cargoExtraArgs = "--features no_downloads"; # cargo-leptos will try to install missing dependencies on its own otherwise
          doCheck = false;
        };

        src = craneLib.cleanCargoSource ../../.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ];
        };

        # Build *just* the cargo dependencies (of the entire workspace),
        # so we can reuse all of that work (e.g. via cachix) when running in CI
        # It is *highly* recommended to use something like cargo-hakari to avoid
        # cache misses when building individual top-level-crates
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Compiled wasm dependencies using crane.
        cargoWasmArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            # wasm-streams wants lld
            nativeBuildInputs = [
              pkgs.lld
            ];

            cargoExtraArgs = "--target=wasm32-unknown-unknown --no-default-features --features hydrate ";
            # Additional environment variables can be set directly
            CARGO_PROFILE = "wasm-release";
            CARGO_TARGET_DIR = "target/front";
          }
        );

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ../../.;
            fileset = lib.fileset.unions [
              ../../Cargo.toml
              ../../Cargo.lock
              (craneLib.fileset.commonCargoSources ../../demo)
              ../../demo/style
              ../../demo/assets
              (craneLib.fileset.commonCargoSources ../../demo/codegen)
              (craneLib.fileset.commonCargoSources ../../overview)
              (craneLib.fileset.commonCargoSources ../../overview/codegen)
              (craneLib.fileset.commonCargoSources ../../proc-macros)
              (craneLib.fileset.commonCargoSources ../../leptodon)
              (craneLib.fileset.commonCargoSources crate)
            ];
          };

        # Builds (release mode + compression) the wasm bin and assets using cargo-leptos.
        buildLeptosWasmPackage =
          pname: sourcePath:
          craneLib.buildPackage (
            individualCrateArgs
            // {
              inherit cargoWasmArtifacts;
              inherit pname;
              nativeBuildInputs = [
                pkgs.lld
                pkgs.wasm-bindgen-cli_0_2_108
                pkgs.binaryen
                cargo-leptos
                craneLib.installFromCargoBuildLogHook
              ];
              buildInputs = [ ];

              LEPTOS_HASH_FILES = "true";
              LEPTOS_LIB_CARGO_STDOUT_PATH = "front_build.log";
              LEPTOS_LIB_CARGO_ARGS = "--message-format json-render-diagnostics";
              RUST_BACKTRACE = "1";

              buildPhaseCargoCommand = ''
                cargoBuildLog=target/front_build.log

                cargo leptos build --frontend-only --release -P -p ${pname};
              '';
              postInstall = ''
                cp -r target/release/hash.txt $out/lib/hash.txt
                cp -r target/site $out/lib/site
              '';

              src = fileSetForCrate sourcePath;
            }
          );

        # Builds (release mode + compression) the server binary using cargo-leptos.
        buildLeptosServerPackage =
          pname: sourcePath:
          craneLib.buildPackage (
            individualCrateArgs
            // {
              inherit cargoWasmArtifacts;
              inherit pname;
              nativeBuildInputs = [
                pkgs.lld
                cargo-leptos
                craneLib.installFromCargoBuildLogHook
              ];

              RUST_BACKTRACE = "1";
              LEPTOS_HASH_FILES = "true";
              LEPTOS_BIN_CARGO_ARGS = "--message-format json-render-diagnostics";
              LEPTOS_BIN_CARGO_STDOUT_PATH = "server_build.log";
              buildPhaseCargoCommand = ''
                cargoBuildLog=target/server_build.log

                cargo leptos build --server-only --release -P -p ${pname};
              '';

              meta.mainProgram = pname;
              src = fileSetForCrate sourcePath;
            }
          );

        # The overview is used for CI-tests, provided via the overview-site derivation.
        overview-wasm = buildLeptosWasmPackage "overview" ../../overview;
        overview-server = buildLeptosServerPackage "overview" ../../overview;
        overview-site = pkgs.writeShellScriptBin "overview-site" ''
          LEPTOS_SITE_ADDR="''${LEPTOS_SITE_ADDR:-0.0.0.0:8080}"
          LEPTOS_SITE_ROOT="''${LEPTOS_SITE_ROOT:-${overview-wasm}/lib/site}"
          LEPTOS_STYLE_FILE="''${LEPTOS_STYLE_FILE:-${overview-wasm}/lib/style/output.css}"
          LEPTOS_HASH_FILE_NAME="''${LEPTOS_HASH_FILE_NAME:-${overview-wasm}/lib/hash.txt}"
          LEPTOS_HASH_FILES="''${LEPTOS_HASH_FILES:-true}"
          export LEPTOS_SITE_ADDR
          export LEPTOS_SITE_ROOT
          export LEPTOS_HASH_FILES
          export LEPTOS_STYLE_FILE
          export LEPTOS_HASH_FILE_NAME
          ${lib.getExe overview-server} "$@"
        '';

        # The demo is hosted on leptodon.dev, provided via the docker image below, published via skopeo.
        demo-wasm = buildLeptosWasmPackage "demo" ../../demo;
        demo-server = buildLeptosServerPackage "demo" ../../demo;
        demo-site-image = pkgs.dockerTools.buildImage {
          name = "demo-site";
          tag = "latest";
          includeNixDB = false;
          copyToRoot = [
            (pkgs.buildEnv {
              name = "image-root";
              pathsToLink = [ "/bin" ];
              paths = [
                demo-server
                demo-wasm
              ];
            })
          ];

          config = {
            Env = [
              "LEPTOS_SITE_ADDR=0.0.0.0:8080"
              "LEPTOS_SITE_ROOT=${demo-wasm}/lib/site"
              "LEPTOS_STYLE_FILE=${demo-wasm}/lib/style/output.css"
              "LEPTOS_HASH_FILE_NAME=${demo-wasm}/lib/hash.txt"
              "LEPTOS_HASH_FILES=true"
            ];
            Cmd = [ "${demo-server}/bin/demo-site" ];
          };
        };

        # Testing derivation, produces junit.xml to be consumed by Jenkins to show test results and quantity.
        nextest = craneLib.cargoNextest (
          commonArgs
          // {
            RUST_BACKTRACE = "full";
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            postInstall = ''
              cp target/nextest/default/junit.xml $out/junit.xml
            '';
          }
        );
      in
      {
        checks = {
          # Build the crates as part of `nix flake check` for convenience
          # inherit demo-wasm demo-server demo-site leptodon leptodon-proc-macros;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-workspace-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          my-workspace-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              # This can be commented out or tweaked as necessary, e.g. set to
              # `--deny rustdoc::broken-intra-doc-links` to only enforce that lint
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          # Check formatting
          my-workspace-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          my-workspace-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Can't get test output from within the flake check :/
          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on other crate derivations
          # if you do not want the tests to run twice

          # # Ensure that cargo-hakari is up to date
          # my-workspace-hakari = craneLib.mkCargoDerivation {
          #   inherit src;
          #   pname = "my-workspace-hakari";
          #   cargoArtifacts = null;
          #   doInstallCargoArtifacts = false;

          #   buildPhaseCargoCommand = ''
          #     cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
          #     cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
          #     cargo hakari verify
          #   '';

          #   nativeBuildInputs = [
          #     pkgs.cargo-hakari
          #   ];
          # };
        };

        packages = {
          inherit
            cargoArtifacts
            cargoWasmArtifacts
            demo-site-image
            overview-site
            demo-wasm
            demo-server
            nextest
            ;
        };

        apps = {
          overview = flake-utils.lib.mkApp {
            drv = overview-site;
          };
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            overview-site
          ];
        };
      }
    );
}
