{
  description = "A database for tracking character sheets from various TTRPGs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      naersk,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        name = "mellon-book";
        src = ./.;
        pkgs = import nixpkgs {
          inherit system;
        };
        naerskLib = pkgs.callPackage naersk { };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            bacon
            cargo
            clippy
            git
            openssl
            rustc
            rustfmt
            rust-analyzer
            sqlite
            sqlx-cli
          ];

          nativeBuildInputs = [
            pkg-config
          ];

          env = {
            RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
          };

          shellHook = ''
            export DATABASE_URL="sqlite://$(git rev-parse --show-toplevel)/database.db";
          '';
        };

        packages = {
          default = naerskLib.buildPackage {
            inherit system name src;
            buildInputs = [
              openssl
              sqlite
            ];
            nativeBuildInputs = [
              pkg-config
            ];
          };
        };
      }
    );
}
