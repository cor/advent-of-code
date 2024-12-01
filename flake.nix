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
  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          crane = rec {
            lib = self.inputs.crane.lib.${system};
            stable = lib.overrideToolchain self'.packages.rust-stable;
          };
          days = map (pkgs.lib.fixedWidthNumber 2) (pkgs.lib.range 1 15);
          days2023 = map (pkgs.lib.fixedWidthNumber 2) (pkgs.lib.range 1 1);
        in {
          packages = {
            new-day = pkgs.writeShellApplication {
                  name = "new-day";
                  runtimeInputs = [ self'.packages.rust-stable ];
                  text = ''
                    cd "$1"
                    cargo new "$2" --name="aoc-$1-$2"

                    echo "aoc-$1-common = { path = \"../common/\" }" >> ./"$2"/Cargo.toml

                    mkdir "$2"/input

                    touch "$2"/input/example.txt
                    touch "$2"/input/1.txt

                    echo "use aoc_$1_common::challenge_input;

                    fn main() {
                        let input = challenge_input();
                        println!(\"{input}\");
                    }" > ./"$2"/src/main.rs

                    echo "Cargo project created!";
                    echo "You should add $2 to ./$1/Cargo.toml";
                    echo "and also add $2 to days in ./flake.nix";
                  '';
                };

            rust-stable = inputs'.rust-overlay.packages.rust.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            };
          # TODO: generate for each year
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

          }) days))
          // (builtins.listToAttrs (map (day: {
            name = "2023-${day}";
            value = let
              build = let pname = "aoc-2023-${day}"; in crane.stable.buildPackage {
                src = ./2023;
                cargoBuildCommand = "cargo build --release -p ${pname}";
                version = "0.1.0";
                inherit pname;
              };
            in pkgs.writeShellApplication {
              name = "aoc-2023-${day}";
              text = ''
                ${build}/bin/aoc-2023-${day} "$@"
              '';
            };

          }) days2023));
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-stable self'.packages.new-day ]
                ++ (with pkgs; [ bacon nil hyperfine cargo-flamegraph lldb llvmPackages_14.llvm ]);
            };
          };
        };
    };
}
