{
  description = "Rust development nix flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ 
          (import rust-overlay) 
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
      in {
        stdenv = pkgs.fastStdenv;
        devShell = pkgs.mkShell {
          LIBCLANG_PATH = pkgs.libclang.lib + "/lib/";
          PROTOC = pkgs.protobuf + "/bin/protoc"; 

          nativeBuildInputs = with pkgs; [
            bashInteractive
            taplo
            clang
            cmake
            openssl
            pkg-config
            # clang
            llvmPackages_11.bintools
            llvmPackages_11.libclang
            protobuf

            nodejs
            solc

          ];
          buildInputs = with pkgs; [
              (rustVersion.override { extensions = [ "rust-src" ]; })
          ];

        };
  });
}
