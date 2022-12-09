{
  description = "Advent of Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, rust-overlay, flake-parts, ... }: flake-parts.lib.mkFlake { inherit self; } {
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    perSystem = { config, self', inputs', pkgs, system, ... }: {
      packages = let 
        crane = rec {
          lib = self.inputs.crane.lib.${system};
          stable = lib.overrideToolchain self'.packages.rust-stable;
        };
      in {
        rust-stable = inputs'.rust-overlay.packages.rust.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };
        
        # TODO map for every day
        "2022-08" = let build = crane.stable.buildPackage {
          src = ./2022;
          cargoBuildCommand = "cargo build --release -p aoc-2022-08";
        }; in pkgs.writeShellApplication {
          name = "2022-08";
          text = ''
            ${build}/bin/aoc-2022-08 "$@"
          '';
        };
      };
      devShells = {
        default =
          pkgs.mkShell {
            buildInputs = [ self'.packages.rust-stable ] ++ (with pkgs; [ bacon rnix-lsp ]);
          };
      };
    };
  };
}
