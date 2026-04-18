
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
        };

        rustToolchain = pkgs.rust-bin.stable."1.89.0".default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "rust-analyzer"
          ];
        };

        commonEnv = {
          ATCODER_PROBLEMS_BASE_ENDPOINT = "https://kenkoooo.com/atcoder";
          ATCODER_PROBLEMS_DIFFICULTY_ENDPOINT =
            "https://v8a0p8685g.execute-api.ap-northeast-1.amazonaws.com/dev";
          ENV = "dev";
          FIREBASE_PROJECT_ID = "atcoder-solutions";

          DATABASE_HOST = "localhost";
          DATABASE_NAME = "app";
          DATABASE_PASSWORD = "password";
          DATABASE_PORT = "5432";
          DATABASE_USERNAME = "app";

          IMAGE_BASE = "asia-northeast1-docker.pkg.dev/atcoder-solutions/atcoder-solutions";
        };

        shellHook = ''
          export DATABASE_URL="postgresql://$DATABASE_HOST:$DATABASE_PORT/$DATABASE_NAME?user=$DATABASE_USERNAME&password=$DATABASE_PASSWORD"
        '';

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
        ];

        frontendTools = with pkgs; [
          nodejs_22
          pnpm
        ];

        infraTools = with pkgs; [
          nodejs_22
          pnpm
          google-cloud-sdk
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            packages = rootTools ++ backendTools ++ frontendTools ++ infraTools;
            env = commonEnv;
            inherit shellHook;
          };

          backend = pkgs.mkShell {
            packages = rootTools ++ backendTools;
            env = commonEnv;
            inherit shellHook;
          };

          frontend = pkgs.mkShell {
            packages = rootTools ++ frontendTools;
            env = commonEnv;
            inherit shellHook;
          };

          infra = pkgs.mkShell {
            packages = rootTools ++ infraTools;
            env = commonEnv;
            inherit shellHook;
          };
        };
      }
    );
}
