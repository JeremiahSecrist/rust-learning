{rustPlatform,
 lib,
 libGL,
 bzip2,
 fontconfig,
 freetype,
 libX11,
 libXcursor,
 libXrandr,
 libXi,
 libxkbcommon,
 vulkan-loader,
 expat,
 gnome,
 libsForQt5,
 cmake,
 pkg-config,
 makeWrapper
 }:
 let
  runtimeLibs = [
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    vulkan-loader
    libX11
    libXcursor
    libXi
    libXrandr
    libxkbcommon
  ];
in
rustPlatform.buildRustPackage {
  pname = "hello_world";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
  postFixup =
      # patchelf --set-rpath "${libPath}" "$out/bin/$pname"
    ''
      wrapProgram  $out/bin/$pname --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath runtimeLibs}"
    '';
  nativeBuildInputs = [
    cmake
    pkg-config
    makeWrapper
  ];

  buildInputs = [
    fontconfig
    # systemd
  ];

  #  doCheck = false;
}
