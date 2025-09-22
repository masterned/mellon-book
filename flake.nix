{
  description = "A database for tracking character sheets from various TTRPGs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    { nixpkgs, naersk, ... }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naerskLib = pkgs.callPackage naersk { };
    in
    {

      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = with pkgs; [
          atlas
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
          terraform-ls
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        env = {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        shellHook = ''
          export DATABASE_URL="sqlite://$(git rev-parse --show-toplevel)/database.db";
        '';
      };

      packages."x86_64-linux".default = naerskLib.buildPackage {
        name = "mellon-book";
        src = ./.;
        buildInputs = with pkgs; [
          openssl
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };
    };
}
