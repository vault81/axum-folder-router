{
  description = "A Nix-flake-based Rust development environment";
  nixConfig = {
    extra-substituters = [
      "https://nixcache.vlt81.de"
    ];
    extra-trusted-public-keys = [
      "nixcache.vlt81.de:nw0FfUpePtL6P3IMNT9X6oln0Wg9REZINtkkI9SisqQ="
    ];
  };
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell.url = "github:numtide/devshell";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    , devshell
    , ...
    }:
    flake-utils.lib.eachDefaultSystem
      (system:
      let
        overlays = [
          rust-overlay.overlays.default
          devshell.overlays.default
          (final: prev: {
            customRustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
          };
        };
        buildInputs = with pkgs; [
          zlib
          clang
          libclang
          gzip
          coreutils
          gdb
          glib
          glibc
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs;
            [
              customRustToolchain
              bacon
              binaryen
              cacert
              cargo-bloat
              cargo-docset
              cargo-machete
              cargo-limit
              cargo-deny
              cargo-edit
              cargo-watch
              cargo-make
              cargo-generate
              cargo-udeps
              cargo-outdated
              cargo-release
              cargo-readme
              cargo-expand
              calc
              fish
              pkg-config
              unzip
            ]
            ++ buildInputs;

          buildInputs = buildInputs;
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}"
            export MALLOC_CONF=thp:always,metadata_thp:always
          '';
        };
      });
}
