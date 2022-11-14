{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-22.05";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      rust = pkgs.rust-bin.nightly.latest.default;
      platform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };
      shellInputs = with pkgs; [
        clang
        (rust.override {extensions = ["rust-src"];})
      ];
      appNativeBuildInputs = with pkgs; [
        pkg-config
      ];
      appBuildInputs =
        appRuntimeInputs
        ++ (with pkgs; [
          openssl
        ]);
      appRuntimeInputs = with pkgs; [
      ];
    in {
      defaultPackage = platform.buildRustPackage {
        src = ./.;
        pname = "proteins";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = appBuildInputs;
        nativeBuildInputs = appNativeBuildInputs;
      };
      devShell = pkgs.mkShell {
        buildInputs = shellInputs ++ appBuildInputs;
        nativeBuildInputs = appNativeBuildInputs;
        shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appBuildInputs}"'';
      };
    });
}