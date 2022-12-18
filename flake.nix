{
  description = "Advent of Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit self; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          crane = rec {
            lib = self.inputs.crane.lib.${system};
            stable = lib.overrideToolchain self'.packages.rust-stable;
          };
          days = map (pkgs.lib.fixedWidthNumber 2) (pkgs.lib.range 1 13);
        in {
          packages = {
            rust-stable = inputs'.rust-overlay.packages.rust.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            };
          } // (builtins.listToAttrs (map (day: {
            name = "2022-${day}";
            value = let
              build = crane.stable.buildPackage {
                src = ./2022;
                cargoBuildCommand = "cargo build --release -p aoc-2022-${day}";
              };
            in pkgs.writeShellApplication {
              name = "aoc-2022-${day}";
              text = ''
                ${build}/bin/aoc-2022-${day} "$@"
              '';
            };

          }) days));
            apps = {
              new-day = {
                type = "app";
                program = pkgs.writeShellApplication {
                  name = "new-day";
                  runtimeInputs = [ self'.packages.rust-stable ];
                  text = ''
                    cd 2022
                    cargo new "$1" --name="aoc-2022-$1"

                    echo "aoc-2022-common = { path = \"../common/\" }" >> ./"$1"/Cargo.toml

                    mkdir "$1"/input

                    touch "$1"/input/example.txt
                    touch "$1"/input/1.txt

                    echo 'use aoc_2022_common::challenge_input;

                    fn main() {
                        let input = challenge_input();
                        println!("{}", input);
                    }' > ./"$1"/src/main.rs

                    echo "Cargo project created!";
                    echo "You should add $1 to ./2022/Cargo.toml";
                    echo "and also add $1 to days in ./flake.nix";
                  '';
                };

              };
            };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-stable ]
                ++ (with pkgs; [ bacon rnix-lsp ]);
            };
          };
        };
    };
}
