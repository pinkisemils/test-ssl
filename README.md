## What's this?
A small sample project to see what linker arguments are required to link
OpenSSL statically on Windows.

## To Build
To build this with bash:
```bash
export OPENSSL_STATIC="1"
export OPENSSL_DIR="$(pwd)/openssl-dir"
cargo build
```
To build with Batch:
```batch
set OPENSSL_STATIC=1
set OPENSSL_DIR=%cd%\openssl-dir
cargo build
```
To build with powershell:
```powershell
$env:OPENSSL_STATIC=1
$env:OPENSSL_DIR=$pwd.Path+"\openssl-dir"
cargo build
```

To see the build fail, checkout the `broken` branch, to see the build succeed,
checkout `fixed`.
