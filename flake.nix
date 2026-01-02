{
  description = "Rust hello world with fenix toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      systems,
      fenix,
      flake-parts,
      self,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;
      perSystem =
        { pkgs, system, ... }:
        let
          toolchain = fenix.packages.${system}.stable.toolchain;
        in
        rec {
          formatter = pkgs.nixfmt-tree;

          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "readall";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ toolchain ];
          };

          packages.readall = packages.default;

          apps.default = {
            type = "app";
            program = "${packages.readall}/bin/readall";
          };

          apps.readall = apps.default;

          devShells.default = pkgs.mkShell {
            packages = [ toolchain ];
          };
        };
    };
}
