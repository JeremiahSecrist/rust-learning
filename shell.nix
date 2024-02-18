{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./package.nix { }) ];
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  # Additional tooling
  buildInputs = with pkgs; [
    cargo
    cargo-watch
    rustc
    rustup
    clippy
    rust-analyzer
    pkg-config
    bacon
    # nodePackages_latest.pnpm
  ];
}