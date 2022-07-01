{
  description = "photo sorting utility";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };

        rustPackage = rustPlatform.buildRustPackage {
          name = "dalia";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "dalia ";
          config = {
            Cmd = [ "${rustPackage}/bin/dalia" ];
          };
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            just
            zsh
          ];
          shellHook = ''
            cat <<EOF
            Welcome to the 🌻dalia development shell.

            EOF
            exec zsh
          '';
        };

        packages = {
          rustPackage = rustPackage;
          dockerImage = dockerImage;
        };
      }
    );
}
