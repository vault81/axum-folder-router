{
  description = "A Nix-flake-based Rust development environment";
  nixConfig = {
    extra-substituters = [
      "https://nixcache.vlt81.de"
      "https://cache.nixos.org"
    ];
    extra-trusted-public-keys = [
      "nixcache.vlt81.de:nw0FfUpePtL6P3IMNT9X6oln0Wg9REZINtkkI9SisqQ="
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
    ];
  };
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";
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
        ];
        customRustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        config = {
          allowUnfree = true;
        };
        pkgs = import nixpkgs {
          inherit system overlays config;
        };
        buildInputs = with pkgs; [
          # pkgsStatic.zstd
          # pkgsStatic.zstd
          # pkgsStatic.zlib   #
          # gzip              #
          # gcc
          # pkgsMusl.musl
          # pkgsMusl.zstd
          # clang
          # libclang
          # coreutils
          # gdb
          # glib
          # glibc
          # psm
        ];
        lib = pkgs.lib;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs;
            [
              customRustToolchain
              bacon
              cacert
              cargo-bloat
              cargo-docset
              cargo-geiger
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
            ]
            ++ buildInputs;

          buildInputs = buildInputs;
          MALLOC_CONF = "thp:always,metadata_thp:always";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";

          shellHook = ''
        '';
        };
      });
}
