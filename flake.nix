{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
  };

  outputs = { self, nixpkgs, naersk, flake-utils, nix-vscode-extensions }: 
  flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {c
      inherit system;
      overlays = [ nix-vscode-extensions.overlays.default ];
      config.allowUnfree = true;
    };
    naerskLib = pkgs.callPackage naersk {};
    rustPackage = naerskLib.buildPackage {
      src = ./.;
      buildInputs = with pkgs; [ ];
      nativeBuildInputs = with pkgs; [ ];
    };
  in {
    packages.default = rustPackage;
    devShells.default = pkgs.mkShell {
      inputsFrom = [ rustPackage ];
      nativeBuildInputs = with pkgs; [
        # VSCode with extensions
        (pkgs.vscode-with-extensions.override {
          vscodeExtensions = with pkgs.vscode-marketplace; [
            rust-lang.rust-analyzer
          ];
        })
      ];
    };
  });
}
