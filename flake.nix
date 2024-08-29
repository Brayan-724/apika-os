{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crane,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    toolchain = fenix.packages.${system}.complete;
    craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

    commonPkgs = with pkgs; [
      # GRUB / iso
      grub2
      xorriso
      nasm
    ];

    commonArgs = {
      src = craneLib.path {path = ./.;};
      strictDeps = true;

      buildInputs = commonPkgs;
      nativeBuildInputs = [];
    };

    cargoArtifacts =
      (craneLib.buildDepsOnly commonArgs)
      // {
        installPhase = "prepareAndInstallCargoArtifactsDir";
      };

    apika-os = craneLib.mkCargoDerivation {
      inherit cargoArtifacts;

      src = craneLib.path ./.;

      pnameSuffix = "-apika-os";

      buildPhaseCargoCommand = ''
        echo "Hola :crabWave:";
      '';

      nativeBuildInputs = [];
    };
    # apika-os = craneLib.buildPackage (commonArgs
    #   // {
    #     inherit cargoArtifacts;
    #   });
  in {
    packages.${system}.default = apika-os;

    # packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
    #   nativeBuildInputs =
    #     commonPkgs
    #     ++ [
    #       (toolchain.withComponents [
    #         "cargo"
    #         # "clippy"
    #         "rust-src"
    #         "rustc"
    #         # "rustfmt"
    #         "rust-analyzer"
    #       ])
    #     ];
    #   buildInputs = commonPkgs;
    #
    #   cargoLock = {
    #     lockFile = ./Cargo.lock;
    #     allowBuiltinFetchGit = true;
    #   };
    #
    #   buildPhase = ''
    #     ls -a
    #     ls .cargo
    #     cat /build/.cargo/config
    #     rm /build/.cargo/config
    #     # mv /build/.cargo/config /build/.cargo/config.toml
    #     echo $PWD
    #     # mkdir -p $out
    #     # cd $out || exit 1
    #     mkdir build || exit 1
    #     cd build || exit 1
    #
    #     # curl https://index.crates.io/config.json
    #     # dig index.crates.io
    #     # nslookup index.crates.io
    #     cargo rustc -- --emit=obj -o apika-os || exit 1
    #     rm build/lib* || exit 1
    #
    #     cd .. || exit 1
    #   '';
    # };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; commonPkgs ++ [qemu];
    };
  };
}
# {
#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
#     fenix.url = "github:nix-community/fenix";
#     flake-utils.url = "github:numtide/flake-utils";
#     crane = {
#       url = "github:ipetkov/crane";
#       inputs.nixpkgs.follows = "nixpkgs";
#     };
#   };
#
#   outputs = {
#     self,
#     nixpkgs,
#     flake-utils,
#     ...
#   } @ inputs:
#     flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
#       system: let
#         bundle = import ./. {
#           inherit system flake-utils;
#           pkgs = nixpkgs.legacyPackages.${system};
#           crane = inputs.crane.lib;
#           fenix = inputs.fenix.packages;
#         };
#       in {
#         inherit (bundle) packages apps devShells;
#       }
#     );
# }
