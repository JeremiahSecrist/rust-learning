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
 vulkan-loader,
 gnome,
 libsForQt5,
 cmake,
 pkg-config,
 makeWrapper
 }:
rustPlatform.buildRustPackage {
  pname = "hello_world";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
  postFixup =
    let
      libPath = lib.makeLibraryPath [
        libGL
        bzip2
        fontconfig
        freetype
        libX11
        libXcursor
        libXrandr
        libXi
        vulkan-loader
      ];
    in
    ''
      patchelf --set-rpath "${libPath}" "$out/bin/$pname"
      wrapProgram $out/bin/$pname --prefix PATH : ${lib.makeBinPath [ gnome.zenity libsForQt5.kdialog ]}
    '';
  nativeBuildInputs = [
    cmake
    pkg-config
    makeWrapper
  ];

  buildInputs = [
    fontconfig
    freetype
    libX11
    libXcursor
    libXrandr
    libXi
  ];

  #  doCheck = false;
}
