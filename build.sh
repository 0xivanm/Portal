#!/usr/bin/env bash
set -e

case "${1:-}" in
    "") deploy=false ;;
    --deploy) deploy=true ;;
    *) echo "Usage: $0 [--deploy]" >&2; exit 1 ;;
esac
if [ "$#" -gt 1 ]; then
    echo "Usage: $0 [--deploy]" >&2
    exit 1
fi

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
firmware=target/armv4t-none-eabi/release/linux.bin
cargo +nightly-2026-10-07 -Z build-std=core build --release --target armv4t-none-eabi --target-dir target
arm-none-eabi-objcopy -O binary target/armv4t-none-eabi/release/ipod "$firmware"

size=$(wc -c < "$firmware")
limit=$((8 * 1024 * 1024))
if [ "$size" -gt "$limit" ]; then
    echo "linux.bin is $size bytes; the bootloader limit is $limit bytes." >&2
    exit 1
fi
echo "Built $firmware ($size bytes)."

if [ "$deploy" = false ]; then
    exit 0
fi

ipod_mount=""
for mount in "/run/media/$USER"/* "/media/$USER"/* /mnt/*; do
    if [ -d "$mount/iPod_Control" ]; then
        ipod_mount="$mount"
        break
    fi
done

if [ -n "$ipod_mount" ]; then
    if cp "$firmware" "$ipod_mount/linux.bin"; then
        echo "Copied linux.bin to $ipod_mount/linux.bin"
    else
        echo "iPod is mounted at $ipod_mount, but linux.bin could not be copied; firmware remains at $firmware"
    fi
else
    echo "iPod is not mounted; firmware remains at $firmware"
fi
