{
  description = "Mark flaky Rust tests";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        rust = fenix.packages."${system}".stable.withComponents [
          "rustc"
          "cargo"
          "rustfmt"
          "rust-src"
          "clippy"
          "rust-analyzer"
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        cargoArtifacts = craneLib.buildDepsOnly { inherit src; };
      in
      {
        checks = {
          test = craneLib.cargoNextest ({
            inherit src cargoArtifacts;
          });

          clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          doc = craneLib.cargoDoc {
            inherit src cargoArtifacts;
          };

          fmt = craneLib.cargoFmt {
            inherit src;
          };

          audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          packages = with pkgs; [
            cargo-readme
            cargo-expand
          ];
        };
      });
}
