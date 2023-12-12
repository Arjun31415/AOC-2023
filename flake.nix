{
  description = "Advent of Code in Rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {nixpkgs, ...} @ inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        system,
        ...
      }: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
        rustPlatform = pkgs.makeRustPlatform {
          rustc = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          cargo = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        };
      in {
        devShells.default = pkgs.mkShell {
          inputsFrom = [config.packages.aoc];
          packages = with pkgs; [
            pre-commit
            vscode-extensions.llvm-org.lldb-vscode
            pkgs.rust-analyzer
          ];

        };
        packages = {
          aoc =
            rustPlatform.buildRustPackage {
              pname = "aoc";
              version = "0.1.0";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            }
            // {default = config.packages.aoc;};
          formatter = pkgs.alejandra;
        };
      };
    };
}
