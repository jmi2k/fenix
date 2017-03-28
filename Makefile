ARCH := i686
TARGET := $(ARCH)-unknown-fenix

all: iso

qemu: iso
	qemu-system-i386 -cdrom out/boot.iso

iso: kernel
	rm -rf build/iso/
	cp -r iso/ build/iso/
	cp out/kernel build/iso/boot/
	grub-mkrescue -o out/boot.iso build/iso/
	cp build/boot.iso out/

kernel: dirs
	xargo build \
		--release \
		--target=$(TARGET)
	ld -n \
		--gc-sections \
		-m elf_i386 \
		-T linker.ld \
		-o build/kernel \
		target/$(TARGET)/release/libfenix.a
	cp build/kernel out/

dirs:
	mkdir -p build/ out/

clean:
	xargo clean
	rm -rf out/ build/ target/
