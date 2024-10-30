{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    fenix,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    toolchain = fenix.packages.${system}.fromToolchainFile {
      file = ./rust-toolchain.toml;
      sha256 = "sha256-NScYMl3lvtfiVqXyjPKc4mAnmwvhn4eAuXEum6bVxR8=";
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        # GRUB / iso
        grub2
        xorriso
        nasm

        # Emulation
        qemu

        # Task
        just

        toolchain
      ];
    };
  };
}
