{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
        };
        formatter = pkgs.nixfmt-tree;
      in
      {
        inherit formatter;
        devShells.default =
          let
            toolchain =
              with inputs.fenix.packages.${system};
              combine [
                (complete.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                  "llvm-tools-preview"
                  "rustc-codegen-cranelift-preview"
                ])

                targets.wasm32-unknown-unknown.latest.rust-std
                targets.x86_64-unknown-linux-gnu.latest.rust-std
                targets.x86_64-pc-windows-gnu.latest.rust-std
              ];
          in
          (pkgs.mkShell.override {
            stdenv = pkgs.llvmPackages_latest.stdenv;
          })
            {
              packages = with pkgs; [
                toolchain
                nixfmt
                nixd
                cargo-autoinherit
                wasm-pack
                watchexec
                pkg-config
                bacon
                cargo-shear
                just
                just-lsp
                upx
                pkgsCross.mingwW64.stdenv.cc
                #pkgsCross.mingwW64.glibc.static
                pkgsCross.mingwW64.binutils
              ];
            };
      }
    );
}
