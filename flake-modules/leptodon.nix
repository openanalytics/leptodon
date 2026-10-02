# Copyright (c) 2022 Ivan Petkov
# Copyright (c) 2026 Open Analytics

{
  self,
  nixpkgs,
  crane,
  flake-utils,
  advisory-db,
  system,
  ...
}:
let
  pkgs = nixpkgs.legacyPackages.${system};
  lib = pkgs.lib;

  craneLib = (crane.mkLib pkgs).overrideScope (
    final: prev: {
      # This is a function which is provided an instance of `pkgs`
      # (which may be tailored for cross compilation, DO NOT reuse
      # the `pkgs` from above`) and returns the `stdenv` instance
      # that should be used across all derivations.
      stdenvSelector = p: p.stdenvAdapters.useMoldLinker p.clangStdenv;
    }
  );

  # Build custom cargo-leptos
  cargo-leptos = craneLib.buildPackage {
    src = pkgs.fetchFromGitHub {
      owner = "ToxicMushroom";
      repo = "cargo-leptos";
      rev = "0951eb8b9a68131100609631bb159f9bf616ab84";
      hash = "sha256-u5dU4dykzB9ChzVJM5ABbRhT9F5nB33J5qTzhSyB+Ew=";
      # At least examples is required to be removed, as crane ends up looking at tomlv1.0 incompatible files there and fails.
      postFetch = ''
        rm -r $out/examples $out/.github $out/.vscode $out/dist-workspace.toml;
      '';
    };
    nativeBuildInputs = [ pkgs.pkg-config ];
    OPENSSL_NO_VENDOR = 1;
    cargoExtraArgs = "--features no_downloads"; # cargo-leptos will try to install missing dependencies on its own otherwise
    doCheck = false;
  };

  src = craneLib.cleanCargoSource ../.;

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;

    # NB: we disable tests since we'll run them all via cargo-nextest
    doCheck = false;
    strictDeps = true;

    nativeBuildInputs = [
      pkgs.lld
      pkgs.pkg-config
      pkgs.binaryen # wasm-opt
      pkgs.wasm-bindgen-cli_0_2_127
      cargo-leptos
    ];

    LEPTOS_HASH_FILES = "true";
    RUST_BACKTRACE = "1";
  };

  # Build *just* the cargo dependencies (of the entire workspace),
  # so we can reuse all of that work (e.g. via cachix) when running in CI
  # It is *highly* recommended to use something like cargo-hakari to avoid
  # cache misses when building individual top-level-crates
  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      pname = "leptodon-server";
    }
  );

  # Compiled wasm dependencies using crane.
  cargoWasmArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      pname = "leptodon-wasm";
      cargoExtraArgs = "--target=wasm32-unknown-unknown --no-default-features --features hydrate ";
      # Additional environment variables can be set directly
      CARGO_PROFILE = "wasm-release";
      CARGO_TARGET_DIR = "target/front";
    }
  );

  mergeWithCommonFileset =
    extraPaths:
    lib.fileset.toSource {
      root = ../.;
      fileset = lib.fileset.unions (
        [
          ../Cargo.toml
          ../Cargo.lock
          # These need to be included since they're listed as workspace dependencies.
          (craneLib.fileset.commonCargoSources ../demo)
          (craneLib.fileset.commonCargoSources ../overview)
          (craneLib.fileset.commonCargoSources ../proc-macros)
          (craneLib.fileset.commonCargoSources ../leptodon)
        ]
        ++ extraPaths
      );
    };

  # Builds (release mode + compression) the wasm bin and assets using cargo-leptos.
  buildLeptosWasmPackage =
    crate: sourceFileSet:
    craneLib.buildPackage (
      commonArgs
      // {
        cargoArtifacts = cargoWasmArtifacts;
        pname = "${crate}-wasm";

        # cargo-leptos owns the install layout; skip crane's default bin copy
        doNotPostBuildInstallCargoBinaries = true;
        buildPhaseCargoCommand = "cargo leptos build --frontend-only --release -P -p ${crate}";

        installPhaseCommand = ''
          mkdir -p $out/lib/
          cp target/release/hash.txt $out/lib/hash.txt
          cp -r target/site $out/lib/site
        '';

        src = sourceFileSet;
      }
    );

  # Builds (release mode + compression) the server binary using cargo-leptos.
  buildLeptosServerPackage =
    crate: sourceFileSet:
    craneLib.buildPackage (
      commonArgs
      // {
        cargoArtifacts = cargoArtifacts;
        pname = "${crate}-server";

        # cargo-leptos owns the install layout; skip crane's default bin copy
        doNotPostBuildInstallCargoBinaries = true;
        buildPhaseCargoCommand = "cargo leptos build --server-only --release -P -vvv -p ${crate}";

        installPhaseCommand = ''
          mkdir -p $out/bin
          ls -al
          cp target/release/${crate} $out/bin/${crate}
        '';

        meta.mainProgram = crate;
        src = sourceFileSet;
      }
    );

  demoFileSet = mergeWithCommonFileset [
    ../demo/style
    ../demo/assets
    (craneLib.fileset.commonCargoSources ../demo/codegen)
  ];

  demoWasm = buildLeptosWasmPackage "demo" demoFileSet;
  demoServer = buildLeptosServerPackage "demo" demoFileSet;

  demoSite = pkgs.writeShellScriptBin "demo-site" ''
    LEPTOS_SITE_ADDR="''${LEPTOS_SITE_ADDR:-0.0.0.0:8080}"
    LEPTOS_SITE_ROOT="''${LEPTOS_SITE_ROOT:-${demoWasm}/lib/site}"
    LEPTOS_HASH_FILE_NAME="''${LEPTOS_HASH_FILE_NAME:-${demoWasm}/lib/hash.txt}"
    LEPTOS_HASH_FILES="''${LEPTOS_HASH_FILES:-true}"
    export LEPTOS_SITE_ADDR
    export LEPTOS_SITE_ROOT
    export LEPTOS_HASH_FILE_NAME
    export LEPTOS_HASH_FILES
    ${lib.getExe demoServer} "$@"
  '';

  # The demo is hosted on leptodon.dev, provided via the docker image below, published via skopeo.
  demoSiteImage = pkgs.dockerTools.buildImage {
    name = "demo-site";
    tag = "latest";
    includeNixDB = false;
    copyToRoot = [
      (pkgs.buildEnv {
        name = "image-root";
        pathsToLink = [
          "/bin"
        ];
        paths = [
          demoWasm
          demoServer
        ];
      })
    ];

    config = {
      Cmd = [ (lib.getExe demoSite) ];
    };
  };

  # Defines file sets relevant for specific projects.
  overviewFileSet = mergeWithCommonFileset [
    ../overview/style
    ../overview/assets
    (craneLib.fileset.commonCargoSources ../overview/codegen)
  ];

  overviewWasm = buildLeptosWasmPackage "overview" overviewFileSet;
  overviewServer = buildLeptosServerPackage "overview" overviewFileSet;
  overviewSite = pkgs.writeShellScriptBin "overview-site" ''
    LEPTOS_SITE_ADDR="''${LEPTOS_SITE_ADDR:-0.0.0.0:8080}"
    LEPTOS_SITE_ROOT="''${LEPTOS_SITE_ROOT:-${overviewWasm}/lib/site}"
    LEPTOS_HASH_FILE_NAME="''${LEPTOS_HASH_FILE_NAME:-${overviewWasm}/lib/hash.txt}"
    LEPTOS_HASH_FILES="''${LEPTOS_HASH_FILES:-true}"
    export LEPTOS_SITE_ADDR
    export LEPTOS_SITE_ROOT
    export LEPTOS_HASH_FILE_NAME
    export LEPTOS_HASH_FILES
    ${lib.getExe overviewServer} "$@"
  '';

  # Testing derivation, produces junit.xml to be consumed by Jenkins to show test results and quantity.
  nextest = craneLib.cargoNextest (
    commonArgs
    // {
      RUST_BACKTRACE = "full";
      inherit cargoArtifacts;
      doCheck = true;
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
    # inherit demoWasm demoServer demo-site leptodon leptodon-proc-macros;

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

      demoServer
      demoWasm
      demoSite
      demoSiteImage

      overviewServer
      overviewWasm
      overviewSite
      nextest
      ;
  };

  apps = {
    demo = flake-utils.lib.mkApp {
      drv = demoSite;
    };
    overview = flake-utils.lib.mkApp {
      drv = overviewSite;
    };
  };

  devShells.default = craneLib.devShell {
    # Inherit inputs from checks.
    checks = self.checks.${system};

    # Additional dev-shell environment variables can be set directly
    # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

    # Extra inputs can be added here; cargo and rustc are provided by default.
    packages = [];
  };
}
