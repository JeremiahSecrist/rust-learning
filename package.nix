{rustPlatform, lib }:
rustPlatform.buildRustPackage {
  pname = "hello_world";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
  buildInputs = [];
  nativeBuildInputs = [];
  #  doCheck = false;
}
