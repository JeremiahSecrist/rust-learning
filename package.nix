{rustPlatform,
 lib,
 fontconfig,
 cmake,
 pkg-config,
 makeWrapper
 }:

rustPlatform.buildRustPackage rec {
  pname = "app";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
  cargoBuildCommand = "cargo build --release --package ${pname}";
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
