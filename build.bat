@echo off
REM Build release version for rust-ffmpeg with vcpkg

call "C:\Programs\Vs\22\VC\Auxiliary\Build\vcvars64.bat"

set VCPKG_ROOT=c:\vcpkg
set PKG_CONFIG_PATH=c:\vcpkg\installed\x64-windows\lib\pkgconfig

echo Building rust-ffmpeg (release) with vcpkg...
cargo build --release %*

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Release build successful!
) else (
    echo.
    echo Build failed with error code %ERRORLEVEL%
)
