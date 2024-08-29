{
  system,
  pkgs,
  lib ? pkgs.lib,
  stdenv ? pkgs.stdenv,
  crane,
  fenix,
  flake-utils,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-fseRtAFO6RK0BL0JKIAbMmCdOlXkYAynYcNmHPd5byQ=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.${system}.overrideToolchain toolchain;

  nativeBuildInputs = with pkgs; [
    # GRUB / iso
    grub2
    xorriso
    nasm
  ];

  buildInputs = with pkgs; [
    qemu
  ];

  apikaOSIso = craneLib.buildPackage {
    doCheck = false;
    src = craneLib.cleanCargoSource (craneLib.path ./.);
    buildPhaseCargoCommand = ''
      ls
      ls $src
      ls $out
      mkdir -p $out
      cd $out || exit 1
      mkdir $out/build || exit 1
      # cd $out/build || exit 1

      rm $out 
      cargo rustc -- --emit=obj -o apika-os || exit 1
      rm $out/build/lib* || exit 1

      cd $out || exit 1
    '';

    installPhaseCommand = ''
      cp -r ./build $out
    '';

    inherit nativeBuildInputs buildInputs;
  };
in {
  # `nix build`
  packages.default = apikaOSIso;

  # `nix develop`
  devShells.default = craneLib.devShell {
    buildInputs = nativeBuildInputs ++ buildInputs;
  };
}
