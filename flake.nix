{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    systems.url = "github:nix-systems/default-linux";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } (
      {
        inputs,
        ...
      }:
      {
        imports = [ inputs.treefmt-nix.flakeModule ];

        systems = import inputs.systems;

        perSystem =
          {
            system,
            pkgs,
            self',
            config,
            ...
          }:
          let
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
              p:
              p.rust-bin.nightly.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
              }
            );

            # Keep the files consumed by the build script in addition to the
            # normal Cargo sources.
            sourceRoots = [
              ./assets
              ./resume
              ./typst
            ];
            src = pkgs.lib.cleanSourceWith {
              src = ./.;
              filter =
                path: type:
                craneLib.filterCargoSources path type
                || pkgs.lib.any (root: pkgs.lib.hasPrefix (toString root) (toString path)) sourceRoots
                || pkgs.lib.hasSuffix "/LICENSE.txt" (toString path)
                || pkgs.lib.elem (pkgs.lib.baseNameOf (toString path)) [
                  "deny.toml"
                  "rustfmt.toml"
                  "taplo.toml"
                ];
            };
            commonArgs = {
              inherit src;
              strictDeps = true;
              nativeBuildInputs = [ pkgs.tailwindcss_4 ];
              cargoExtraArgs = "--all-features --locked";
            };
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          in
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };

            packages.default = craneLib.buildPackage (
              commonArgs
              // {
                inherit cargoArtifacts;
                meta.mainProgram = "vidhan-site";
              }
            );

            checks = {
              clippy = craneLib.cargoClippy (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--all-targets -- -D warnings";
                }
              );

              test = craneLib.cargoTest (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  cargoTestExtraArgs = "--all-targets";
                }
              );

              fmt = craneLib.cargoFmt { inherit src; };

              deny = craneLib.cargoDeny { inherit src; };

              nextest = craneLib.cargoNextest (
                commonArgs
                // {
                  inherit cargoArtifacts;
                  partitions = 1;
                  partitionType = "count";
                  cargoNextestPartitionsExtraArgs = "--no-tests=pass";
                }
              );
            };

            devShells.default = craneLib.devShell {
              inherit (self') checks;

              env.CARGO_NET_GIT_FETCH_WITH_CLI = "true";

              packages = [
                pkgs.cargo-deny
                pkgs.cargo-edit
                pkgs.nil
                pkgs.prek
                pkgs.tailwindcss_4
                config.treefmt.build.wrapper
              ];
            };

            treefmt = {
              programs = {
                nixfmt.enable = true;
                statix.enable = true;
                deadnix.enable = true;
                rustfmt = {
                  enable = true;
                  package = pkgs.rust-bin.nightly.latest.rustfmt;
                };
                taplo.enable = true;
              };
            };
          };
      }
    );
}
