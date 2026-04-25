{
  description = "swift-bridge development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          swift
          swiftPackages.swiftpm
          swiftPackages.XCTest
        ];

        shellHook = ''
          export CARGO_BUILD_TARGET=aarch64-apple-darwin
          export PATH="$PATH:/usr/bin:/bin"
        '';
      };
    };
}
