#!/bin/bash

# Define paths and mappings
local_bin_name="AndroidMemoryTools"
device_path="/data/local/tmp/$local_bin_name"
declare -A targets=(
    ["armv7"]="armv7-linux-androideabi"
    ["arm64"]="aarch64-linux-android"
    ["x86"]="i686-linux-android"
    ["x86_64"]="x86_64-linux-android"
)

# 1. Check ADB device connection
echo "=== Checking device connection ==="
devices=$(adb devices | grep -w "device$")
if [ -z "$devices" ]; then
    echo "Error: No Android device connected!" >&2
    exit 1
fi

# 2. Detect device architecture
echo -e "\n=== Detecting device architecture ==="
device_abi=$(adb shell getprop ro.product.cpu.abi | tr -d '\r\n')
matched_arch=""

for arch in "${!targets[@]}"; do
    if [[ "$device_abi" == *"$arch"* ]]; then
        matched_arch="$arch"
        break
    fi
done

if [ -z "$matched_arch" ]; then
    echo "Error: Unsupported device ABI: $device_abi" >&2
    exit 1
fi

echo "Detected architecture: $matched_arch ($device_abi)"

# 3. Compile for target architecture
echo -e "\n=== Compiling for $matched_arch ==="
rust_target=${targets[$matched_arch]}
cargo build --target "$rust_target" --release
if [ $? -ne 0 ]; then
    echo "Error: Compilation failed!" >&2
    exit 1
fi

# 4. Push the binary
echo -e "\n=== Pushing binary ==="
local_path="../target/$rust_target/release/$local_bin_name"
remote_path="$device_path-$matched_arch"

if [ -f "$local_path" ]; then
    echo "[$matched_arch] Pushing to $remote_path"
    adb push "$local_path" "$remote_path"
    adb shell chmod 777 "$remote_path"

    # Verify push success
    remote_size=$(adb shell "stat -c%s \"$remote_path\" 2>/dev/null || echo 0")
    if [ "$remote_size" -eq 0 ]; then
        echo "Error: Failed to push binary!" >&2
        exit 1
    fi
    echo "Push successful. File size: $remote_size bytes"
else
    echo "Error: Binary not found at $local_path" >&2
    echo "Contents of target directory:"
    ls -lh "../target/$rust_target/release"
    exit 1
fi

# 5. Run the binary
echo -e "\n=== Executing binary ==="
echo "Running: $remote_path"
adb shell su -c "$remote_path"