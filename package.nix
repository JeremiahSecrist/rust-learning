{rustPlatform,
 lib,
 glibc,
 craneLib
#  pkgsMusl
#  fontconfig,
#  cmake,
#  pkg-config,
#  llvmPackages,
#  clang,
#  openssl,
#  makeWrapper
 }:

 craneLib.buildPackage rec {
  pname = "app";
  version = "0.1.0";
  # cargoLock.lockFile = ./Cargo.lock;
  src = craneLib.cleanCargoSource (craneLib.path ./.);
  cargoBuildCommand = "cargo build --release --package ${pname}";

  CARGO_INCREMENTAL = "0";
  # RUST_BACKTRACE=1;
  # LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib";
  strictDeps = true;
  # CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
  CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
  buildInputs = [
    glibc
    glibc.static
  ];
  #  doCheck = false;
}
