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
          bacon
          cargo
          clippy
          rustc
          rustfmt
          rust-analyzer
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      packages."x86_64-linux".default = naerskLib.buildPackage {
        name = "mellon-book";
        src = ./.;
      };
    };
}
