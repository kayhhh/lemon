{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = lib.cleanSource ./.;

          strictDeps = true;

          buildInputs = with pkgs;
            [ openssl ] ++ lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.libiconv
            ];

          nativeBuildInputs = with pkgs; [
            cargo-auditable
            nodePackages.prettier
            ollama
            pkg-config
          ];
        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [ cargo-watch rust-analyzer ];
        };

        cargoArtifacts =
          craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          pname = "clippy";
        });

        cargoDoc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
          pname = "doc";
        });

        lemon-agent = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "lemon-agent";
        });

        lemon-graph = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "lemon-graph";
        });

        lemon-llm = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "lemon-llm";
        });

        lemon-memory = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "lemon-memory";
        });
      in {
        checks = {
          inherit lemon-agent lemon-graph lemon-llm lemon-memory cargoClippy
            cargoDoc;
        };

        packages = rec {
          agent = lemon-agent;
          graph = lemon-graph;
          llm = lemon-llm;
          memory = lemon-memory;

          default = pkgs.symlinkJoin {
            name = "all";
            paths = [ agent graph llm memory ];
          };
        };

        devShells = {
          default = craneLib.devShell commonShell;
          ollama = craneLib.devShell (commonShell // {
            shellHook = ''
              ${pkgs.ollama}/bin/ollama serve > /dev/null 2>&1 &
              OLLAMA_PID=$!

              echo "Ollama is running with PID $OLLAMA_PID"

              finish()
              {
                echo "Shutting down Ollama"
                kill -9 $OLLAMA_PID
                wait $OLLAMA_PID
              }

              trap finish EXIT

              $SHELL
            '';
          });
        };
      });
}
