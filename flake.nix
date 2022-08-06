{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay.url = "github:oxalica/rust-overlay";

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, advisory-db, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        commonArgs = {
          src = ./.;

          buildInputs = with pkgs; [
            openssl
            zlib
            stockfish
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "chess-deps";
        });

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        chess = craneLib.buildPackage craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          inherit chess;

          chess-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-- --deny warnings";
          });

          # Check formatting
          chess-fmt = craneLib.cargoFmt commonArgs;

          # Audit dependencies. NOT WORKING WITH PRISMA
          # chess-audit = craneLib.cargoAudit {
          #   inherit src advisory-db;
          # };

          chess-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestExtraArgs = "--workspace";
          });
        } // lib.optionalAttrs (system == "x86_64-linux") {
          chess-coverage = craneLib.cargoTarpaulin (commonArgs // {
            inherit cargoArtifacts;
            cargoTarpaulinExtraArgs = "--workspace";

            CARGO_PROFILE = "dev";
          });
        };

        packages.default = chess;

        apps.default = flake-utils.lib.mkApp {
          drv = chess;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rustfmt" ];
            })

            nodejs
            nodePackages.prisma

            # devtools
            rnix-lsp
            rust-analyzer
          ] ++ commonArgs.buildInputs ++ commonArgs.nativeBuildInputs;

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          PRISMA_MIGRATION_ENGINE_BINARY = "${pkgs.prisma-engines}/bin/migration-engine";
          PRISMA_QUERY_ENGINE_BINARY = "${pkgs.prisma-engines}/bin/query-engine";
          PRISMA_QUERY_ENGINE_LIBRARY = "${pkgs.prisma-engines}/lib/libquery_engine.node";
          PRISMA_INTROSPECTION_ENGINE_BINARY = "${pkgs.prisma-engines}/bin/introspection-engine";
          PRISMA_FMT_BINARY = "${pkgs.prisma-engines}/bin/prisma-fmt";
        };
      });
}
