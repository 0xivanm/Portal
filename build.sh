#!/usr/bin/env bash
set -e

cargo -Z build-std=core build --release --target armv4t-none-eabi
arm-none-eabi-objcopy -O binary target/armv4t-none-eabi/release/ipod linux.bin

ipod_mount=""
for mount in "/run/media/$USER"/* "/media/$USER"/* /mnt/*; do
    if [ -d "$mount/iPod_Control" ]; then
        ipod_mount="$mount"
        break
    fi
done

if [ -n "$ipod_mount" ]; then
    if cp linux.bin "$ipod_mount/linux.bin"; then
        echo "Copied linux.bin to $ipod_mount/linux.bin"
    else
        echo "iPod is mounted at $ipod_mount, but linux.bin could not be copied; linux.bin was left in the project root"
    fi
else
    echo "iPod is not mounted; linux.bin was left in the project root"
fi
