{
  description = "kinugasa-mocap development flake";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs@{
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        ./treefmt.nix
        ./rust-toolchain.nix
      ];

      perSystem =
        {
          pkgs,
          config,
          ...
        }:
        {
          devShells.default = pkgs.mkShell {
            name = "devshell";

            packages = with pkgs; [
              config.packages.rust-toolchain

              libclang.lib
              meson
              pkg-config
              ninja

              treefmt
              config.packages."ci:treefmt:sync"
            ];

            shellHook = ''
              treefmt-sync
              export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            '';
          };
        };
    };
}
