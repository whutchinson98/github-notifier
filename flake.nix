{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          system = system;
        };
        packages = with pkgs; [
          cargo-info
          cargo-udeps
          just
          (
            with fenix.packages.${system};
            combine [
              complete.rustc
              complete.rust-src
              complete.cargo
              complete.clippy
              complete.rustfmt
              complete.rust-analyzer
            ]
          )
        ];

        libraries = with pkgs; [
          openssl
          pkg-config
        ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "github-notifier";
          version = "1.0.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          meta = with pkgs.lib; {
            description = "A CLI tool to notify github of new commits";
            homepage = "https://github.com/whutchinson98/github-notifier";
            license = licenses.mit;
            maintainers = [ ];
            mainProgram = "github-notifier";
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = packages ++ libraries;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}
