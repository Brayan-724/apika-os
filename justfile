run: build
  qemu-system-x86_64 apika-os.iso

build: clean
  @mkdir -p build

  @echo "------------------------------"
  @echo "      Compiling Kernel"
  @echo "------------------------------"
  cargo rustc -- --emit=obj -o apika-os
  @mv apika-os*.o build/kernel.o
  @rm apika-os* libapika_os* &> /dev/null

  @echo "------------------------------"
  @echo "        Building GRUB"
  @echo "------------------------------"
  nasm -f elf64 grub/boot.S -o build/grub.o

  ld -m elf_x86_64 -T grub/linker.ld -o build/kernel build/grub.o build/kernel.o

  @mkdir -p iso/boot/grub

  cp grub/grub.cfg iso/boot/grub/
  cp build/kernel iso/boot/kernel

  grub-mkrescue -o apika-os.iso iso

  @echo "------------------------------"
  @echo "          Finished"
  @echo "------------------------------"

clean:
  @rm -rf build || echo "No build folder"
