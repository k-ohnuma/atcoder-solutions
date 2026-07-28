{
  description = "Development shells for atcoder-solutions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];

          config.allowUnfreePredicate =
            pkg:
            builtins.elem (nixpkgs.lib.getName pkg) [
              "terraform"
            ];
        };

        rustToolchain = pkgs.rust-bin.stable."1.89.0".default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "rust-analyzer"
          ];
        };

        rootTools = with pkgs; [
          just
          taplo
          yamlfmt
        ];

        backendTools = with pkgs; [
          rustToolchain
          cargo-nextest
          sqlx-cli
          pkg-config
          openssl
          cargo-audit
        ];

        frontendTools = with pkgs; [
          nodejs_24
          pnpm
        ];

        infraTools = with pkgs; [
          nodejs_24
          pnpm
          google-cloud-sdk
          cdktn-cli
          terraform
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            packages = rootTools ++ backendTools ++ frontendTools ++ infraTools;
          };

          backend = pkgs.mkShell {
            packages = rootTools ++ backendTools;
          };

          frontend = pkgs.mkShell {
            packages = rootTools ++ frontendTools;
          };

          infra = pkgs.mkShell {
            packages = rootTools ++ infraTools;
          };
        };
      }
    );
}
