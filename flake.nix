{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        # set up nixpkgs with rust overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        # set up the rust package
        rust-bin = pkgs.rust-bin.stable.latest.default.override {
          # set the target for stm32f411
          targets = ["thumbv7em-none-eabihf"];

          # include useful extensions
          extensions = [
            "rust-analyzer"
          ];
        };
      in {
        # build default rust dev shell
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              pkg-config
              rust-bin
            ];

            RUST_SRC_PATH = "${rust-bin}/lib/rustlib/src/rust/library";
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      }
    );
}
