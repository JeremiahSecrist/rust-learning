{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "hello_world";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  buildInputs = [ ];
  nativeBuildInputs = [ ];
#  doCheck = false;
}