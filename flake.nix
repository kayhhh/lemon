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

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          pname = "lemon";
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;

          buildInputs =
            with pkgs;
            [ openssl ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              libiconv
            ];

          nativeBuildInputs = with pkgs; [
            cargo-auditable
            nodePackages.prettier
            ollama
            pkg-config
            qdrant
          ];
        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [
            cargo-machete
            cargo-rdme
            rust-analyzer
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });
        cargoClippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; });
        cargoDoc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });
        cargoFmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });
        cargoTest = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });

        lemon-agent = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "lemon-agent";
          }
        );

        lemon-graph = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "lemon-graph";
          }
        );

        lemon-llm = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "lemon-llm";
          }
        );

        lemon-memory = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "lemon-memory";
          }
        );
      in
      {
        checks = {
          inherit
            cargoClippy
            cargoDoc
            cargoFmt
            cargoTest
            ;
        };

        apps = {
          generate-readme = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "generate-readme" ''
              cd crates

              for folder in */; do
                (cd $folder && cargo rdme)
              done
            '';
          };
        };

        packages = {
          lemon-agent = lemon-agent;
          lemon-graph = lemon-graph;
          lemon-llm = lemon-llm;
          lemon-memory = lemon-memory;

          default = pkgs.symlinkJoin {
            name = "all";
            paths = [
              lemon-agent
              lemon-graph
              lemon-llm
              lemon-memory
            ];
          };
        };

        devShells = {
          default = craneLib.devShell commonShell;
          ollama = craneLib.devShell (
            commonShell
            // {
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
            }
          );
          qdrant = craneLib.devShell (
            commonShell
            // {
              shellHook = ''
                ${pkgs.qdrant}/bin/qdrant > /dev/null 2>&1 &
                QDRANT_PID=$!

                echo "Qdrant is running with PID $QDRANT_PID"

                finish()
                {
                  echo "Shutting down Qdrant"
                  kill -9 $QDRANT_PID
                  wait $QDRANT_PID
                }

                trap finish EXIT

                $SHELL
              '';
            }
          );
        };
      }
    );
}
