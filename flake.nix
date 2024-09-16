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
    toolchain = fenix.packages.${system}.complete;
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

        (toolchain.withComponents [
          "cargo"
          "rust-src"
          "rustc"
          "rust-analyzer"
        ])
      ];
    };
  };
}
