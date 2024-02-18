{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = inputs: with inputs; let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    packages.x86_64-linux = rec {
      default = package;
      package = import ./package.nix {inherit pkgs;};
    };
    devShells.x86_64-linux.default = import ./shell.nix {inherit pkgs;};
  };
}
