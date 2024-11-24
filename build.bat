@echo off
echo Building Rust Finder for multiple platforms...

:: Limpiar builds anteriores
echo Limpiando builds anteriores...
cargo clean
rmdir /s /q "target\package" 2>nul
mkdir "target\package"

:: Build para Windows
echo.
echo Building for Windows...
cargo build --release
if %ERRORLEVEL% EQU 0 (
    echo Creando paquete Windows...
    mkdir "target\package\windows"
    copy "target\release\rust_finder.exe" "target\package\windows\"
    copy "assets\icon.ico" "target\package\windows\"
    echo Rust Finder v0.1.0 > "target\package\windows\version.txt"
    echo Build date: %date% %time% >> "target\package\windows\version.txt"
    powershell Compress-Archive -Path "target\package\windows\*" -DestinationPath "target\package\RustFinder-windows.zip" -Force
    echo Windows build completado!
) else (
    echo Error en el build de Windows
)

:: Build para Linux
echo.
echo Building for Linux...
cargo build --release --target x86_64-unknown-linux-gnu
if %ERRORLEVEL% EQU 0 (
    echo Creando paquete Linux...
    mkdir "target\package\linux"
    copy "target\x86_64-unknown-linux-gnu\release\rust_finder" "target\package\linux\"
    copy "assets\icon.ico" "target\package\linux\"
    echo "Rust Finder v0.1.0" > "target\package\linux\version.txt"
    echo "Build date: %date% %time%" >> "target\package\linux\version.txt"
    powershell Compress-Archive -Path "target\package\linux\*" -DestinationPath "target\package\RustFinder-linux.tar.gz" -Force
    echo Linux build completado!
) else (
    echo Error en el build de Linux - Asegúrate de tener instalado el target con: rustup target add x86_64-unknown-linux-gnu
)

:: Build para macOS (Intel)
echo.
echo Building for macOS (Intel)...
cargo build --release --target x86_64-apple-darwin
if %ERRORLEVEL% EQU 0 (
    echo Creando paquete macOS Intel...
    mkdir "target\package\macos-intel"
    copy "target\x86_64-apple-darwin\release\rust_finder" "target\package\macos-intel\"
    copy "assets\icon.icns" "target\package\macos-intel\"
    echo "Rust Finder v0.1.0" > "target\package\macos-intel\version.txt"
    echo "Build date: %date% %time%" >> "target\package\macos-intel\version.txt"
    powershell Compress-Archive -Path "target\package\macos-intel\*" -DestinationPath "target\package\RustFinder-macos-intel.tar.gz" -Force
    echo macOS Intel build completado!
) else (
    echo Error en el build de macOS Intel - Asegúrate de tener instalado el target con: rustup target add x86_64-apple-darwin
)

:: Build para macOS (Apple Silicon/M1)
echo.
echo Building for macOS (Apple Silicon)...
cargo build --release --target aarch64-apple-darwin
if %ERRORLEVEL% EQU 0 (
    echo Creando paquete macOS Apple Silicon...
    mkdir "target\package\macos-arm"
    copy "target\aarch64-apple-darwin\release\rust_finder" "target\package\macos-arm\"
    copy "assets\icon.icns" "target\package\macos-arm\"
    echo "Rust Finder v0.1.0" > "target\package\macos-arm\version.txt"
    echo "Build date: %date% %time%" >> "target\package\macos-arm\version.txt"
    powershell Compress-Archive -Path "target\package\macos-arm\*" -DestinationPath "target\package\RustFinder-macos-arm.tar.gz" -Force
    echo macOS Apple Silicon build completado!
) else (
    echo Error en el build de macOS Apple Silicon - Asegúrate de tener instalado el target con: rustup target add aarch64-apple-darwin
)

echo.
echo Proceso de build completado!
echo Los archivos se encuentran en target\package
echo.

pause