{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs: with inputs; let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
    craneLib = crane.lib.${system};
  in {
    packages.x86_64-linux = rec {
      default = package;
      package = pkgs.callPackage ./package.nix {inherit craneLib;};
    };
    # devShells.x86_64-linux.default = import ./shell.nix {inherit pkgs;};
  };
}
