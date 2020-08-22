rustup run nightly cargo bootimage
qemu-system-x86_64.exe -drive format=raw,file=target/x86_64-CDI/debug/bootimage-CDI.bin -serial stdio