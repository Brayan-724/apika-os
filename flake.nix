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

    cargoArtifacts = let
      deps = craneLib.vendorCargoDeps {
        src = craneLib.path ./.;
        cargoLock = "${toolchain.rust-src}/lib/rustlib/src/rust/library/Cargo.lock";
      };
    in
      pkgs.stdenv.mkDerivation {
        name = "apika-os-deps";

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;

          filter = path: type:
            pkgs.lib.any (suffix: pkgs.lib.hasSuffix suffix path) [
              ".toml"
              ".lock"
              ".json"
            ];
        };

        nativeBuildInputs = [
          (toolchain.withComponents
            [
              "cargo"
              "rust-src"
              "rustc"
            ])
        ];

        buildPhase = let
          dummyrs = pkgs.writeText "dummy.rs" ''
            #![allow(clippy::all)]
            #![allow(dead_code)]
            #![no_std]
            #![no_main]

            #[allow(unused_extern_crates)]
            extern crate core;

            #[panic_handler]
            fn panic(_info: &::core::panic::PanicInfo<'_>) -> ! {
                loop {}
            }

            #[no_mangle]
            extern "C" fn _start() {}
          '';
        in ''
          mkdir /build/.cargo || exit 1
          ln -s ${deps}/config.toml /build/.cargo/config.toml || exit 1

          mkdir -p src
          ln -s ${dummyrs} src/main.rs

          CARGO_TARGET_DIR=/build/target cargo build --target ./x86_64-am-kernel.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem
        '';

        installPhase = ''
          mkdir -p $out
          ln -s ${deps}/config.toml $out/config.toml
          cp -r --no-preserve=mode,ownership /build/target $out/target
        '';
      };

    apika-os = pkgs.stdenv.mkDerivation rec {
      name = "apika-os";

      src = builtins.path {path = ./.;};

      nativeBuildInputs =
        commonPkgs
        ++ [
          (toolchain.withComponents
            [
              "cargo"
              "rust-src"
              "rustc"
            ])
        ];

      buildPhase = ''
        mkdir /build/.cargo || exit 1
        ls ${cargoArtifacts} -al
        cp ${cargoArtifacts}/config.toml /build/.cargo/config.toml || exit 1

        mkdir build

        cp -r --no-preserve=mode,ownership ${cargoArtifacts}/target /build/target
        CARGO_TARGET_DIR=/build/target cargo rustc -- --emit=obj -o apika-os || exit 1
        mv apika-os*.o build/kernel.o

        nasm -f elf64 grub/boot.S -o build/grub.o

        ld -m elf_x86_64 -T grub/linker.ld -o build/kernel build/grub.o build/kernel.o

        mkdir -p iso/boot/grub

        mv grub/grub.cfg iso/boot/grub/
        mv build/kernel.o iso/boot/kernel

        grub-mkrescue -o ${name}.iso iso
      '';

      installPhase = ''
        mkdir -p $out
        cp --no-preserve=mode,ownership ${name}.iso $out/${name}.iso
      '';
    };
  in {
    apps.${system}.default = {
      type = "app";
      program = let
        script = pkgs.writeShellScript "run-apika-os.sh" ''
          cp --no-preserve=mode,ownership ${apika-os}/apika-os.iso apika-os.iso
          ${pkgs.qemu}/bin/qemu-system-x86_64 apika-os.iso
        '';
      in "${script}";
    };

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
