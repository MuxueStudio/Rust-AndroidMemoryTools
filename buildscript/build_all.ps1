# Define paths and mappings
$local_bin_name = "AndroidMemoryTools"
$device_path = "/data/local/tmp/$local_bin_name"
$targets = @{
    "armv7" = "armv7-linux-androideabi"
    "arm64" = "aarch64-linux-android"
    "x86" = "i686-linux-android"
    "x86_64" = "x86_64-linux-android"
}

# 1. Check ADB device connection
# Write-Output "=== Checking device connection ==="
$devices = adb devices | Where-Object { $_ -match "device$" }
if (-not $devices) {
    Write-Error "No Android device connected!"
    exit 1
}

# 2. Detect device architecture
# Write-Output "`n=== Detecting device architecture ==="
$device_abi = (adb shell getprop ro.product.cpu.abi).Trim()
$matched_arch = $targets.Keys | Where-Object { $device_abi -match $_ } | Select-Object -First 1

if (-not $matched_arch) {
    Write-Error "Unsupported device ABI: $device_abi"
    exit 1
}

# Write-Output "Detected architecture: $matched_arch ($device_abi)"

# 3. Compile for target architecture
# Write-Output "`n=== Compiling for $matched_arch ==="
$rust_target = $targets[$matched_arch]
cargo build --target $rust_target --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Compilation failed!"
    exit 1
}

# 4. Push the binary
# Write-Output "`n=== Pushing binary ==="
$local_path = "../target/$rust_target/release/$local_bin_name"
$remote_path = "$device_path-$matched_arch"

if (Test-Path $local_path) {
#     Write-Output "[$matched_arch] Pushing to $remote_path"
    adb push $local_path $remote_path
    adb shell chmod 777 $remote_path

    # Verify push success
    $remote_size = adb shell "stat -c%s $remote_path 2>/dev/null || echo 0"
    if ([int]$remote_size -eq 0) {
#         Write-Error "Failed to push binary!"
        exit 1
    }
#     Write-Output "Push successful. File size: $remote_size bytes"
} else {
    Write-Error "Binary not found at $local_path"
    Get-ChildItem "../target/$rust_target/release" | Format-Table Name, Length
    exit 1
}

# 5. Run the binary
# Write-Output "`n=== Executing binary ==="
# Write-Output "Running: $remote_path"
adb shell su -c $remote_path