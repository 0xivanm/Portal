param([switch]$Deploy)

$ErrorActionPreference = 'Stop'
$target = 'armv4t-none-eabi'
$firmware = "target/$target/release/linux.bin"

Push-Location $PSScriptRoot
try {
    cargo +nightly build -Z build-std=core --release --target $target --target-dir target
    if ($LASTEXITCODE -ne 0) { throw 'Build failed.' }

    arm-none-eabi-objcopy -O binary "target/$target/release/ipod" $firmware
    if ($LASTEXITCODE -ne 0) { throw 'Binary conversion failed.' }

    $size = (Get-Item -LiteralPath $firmware).Length
    $limit = 8 * 1024 * 1024
    if ($size -gt $limit) { throw "linux.bin is $size bytes; the bootloader limit is $limit bytes." }
    Write-Host "Built $firmware ($size bytes)."

    if ($Deploy) {
        $ipod = Get-PSDrive -PSProvider FileSystem |
            Where-Object { Test-Path -LiteralPath (Join-Path $_.Root 'iPod_Control') -PathType Container -ErrorAction SilentlyContinue } |
            Select-Object -First 1

        if ($ipod) {
            $destination = Join-Path $ipod.Root 'linux.bin'
            try {
                Copy-Item -LiteralPath $firmware -Destination $destination -Force
                Write-Host "Copied linux.bin to $destination"
            }
            catch {
                Write-Warning "Copy failed: $_. Firmware remains at $firmware."
            }
        }
        else {
            Write-Host "iPod is not mounted; firmware remains at $firmware."
        }
    }
}
finally {
    Pop-Location
}
