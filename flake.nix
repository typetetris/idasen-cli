{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system: {
        packages.idasen-cli = nixpkgs.legacyPackages.${system}.callPackage ./default.nix { };
        defaultPackage = self.packages.${system}.idasen-cli;
      });
}
