{
  flake-utils,
  nixpkgs,
  self,
  linux-systems,
  ...
}:
flake-utils.lib.eachSystemPassThrough linux-systems (
  system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
    # Read the file relative to the flake's root
    overrides = (fromTOML (builtins.readFile (self + "/rust-toolchain.toml")));
  in
  {
    ${system}.default = pkgs.mkShell rec {
      nativeBuildInputs = [ pkgs.pkg-config ];
      buildInputs = with pkgs; [
        clang
        llvmPackages.bintools
        rustup
        jq
        skopeo
        cargo-hack
        cargo-nextest
        cargo-deny
        zstd
        licensure
      ];

      RUSTC_VERSION = overrides.toolchain.channel;

      # https://github.com/rust-lang/rust-bindgen#environment-variables
      LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

      shellHook = ''
        export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
        export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
        rustup target add wasm32-unknown-unknown
      '';

      # Add precompiled library to rustc search path
      RUSTFLAGS = (
        map (a: "-L ${a}/lib") [
          # add libraries here (e.g. pkgs.libvmi)
        ]
      );

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);

      # Add glibc, clang, glib, and other headers to bindgen search path
      BINDGEN_EXTRA_CLANG_ARGS =
        # Includes normal include path
        (map (a: ''-I"${a}/include"'') [
          # add dev libraries here (e.g. pkgs.libvmi.dev)
          pkgs.glibc.dev
        ])
        # Includes with special directory paths
        ++ [
          ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
          ''-I"${pkgs.glib.dev}/include/glib-2.0"''
          "-I${pkgs.glib.out}/lib/glib-2.0/include/"
        ];
    };
  }
)
