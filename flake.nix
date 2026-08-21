{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { flake-utils, ... }@inputs:
    let
      linux-systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      inputs-with-systems = inputs // {
        inherit linux-systems;
      };
      leptodon-by-system = system: (import ./flake-modules/leptodon.nix (inputs // { inherit system; }));
    in
    {
      devShells = import ./flake-modules/devshell.nix inputs-with-systems;

      apps = flake-utils.lib.eachSystemPassThrough linux-systems (system: {
        ${system} = (leptodon-by-system system).apps;
      });
      packages = flake-utils.lib.eachSystemPassThrough linux-systems (system: {
        ${system} = (leptodon-by-system system).packages;
      });
      checks = flake-utils.lib.eachSystemPassThrough linux-systems (system: {
        ${system} = (leptodon-by-system system).checks;
      });
    };
}
