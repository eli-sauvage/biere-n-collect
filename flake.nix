{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      rustVersion = pkgs.rust-bin.stable.latest.default;
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          (rustVersion.override {extensions = ["rust-src"];})
          #rust-analyzer
          openssl
          pkg-config
          sqlx-cli
          lld
          pre-commit
        ];
        shellHook = ''
          pre-commit install
        '';
      };
    });
}
