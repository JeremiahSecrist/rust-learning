{rustPlatform,
 lib,
 fontconfig,
 cmake,
 pkg-config,
 makeWrapper
 }:

rustPlatform.buildRustPackage {
  pname = "hello_world";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
  nativeBuildInputs = [
    cmake
    pkg-config
    makeWrapper
  ];
  
  CARGO_INCREMENTAL = "0";
  buildInputs = [
    fontconfig
  ];

  #  doCheck = false;
}
