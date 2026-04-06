{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        formatter = pkgs.nixfmt-tree;
      in
      {
        inherit formatter;
        devShells.default =
          (pkgs.mkShell.override {
            stdenv = pkgs.llvmPackages_latest.stdenv;
          })
            {
              packages = with pkgs; [
                nixfmt
                nixd
                cargo-autoinherit
                rustup
                wasm-pack
                watchexec
                pkg-config
                bacon
                cargo-shear
                just
                just-lsp
                upx
              ];
            };
      }
    );
}
