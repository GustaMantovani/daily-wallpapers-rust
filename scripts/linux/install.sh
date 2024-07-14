#!/bin/bash

# Build the Rust project
echo "Building the Rust project..."
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Build succeeded!"

# Building external dependencies
./scripts/linux/build_wallpaper_changer.sh

# Check if the build of external dependencies was successful
if [ $? -ne 0 ]; then
    echo "Build dependencies failed!"
    exit 1
fi

echo "Build dependencies succeeded!"

# Create the .dwr/bin directory in the user's home directory
DWR_DIR="$HOME/.dwr/bin"
mkdir -p "$DWR_DIR"

# Move the executable to the .dwr/bin directory
EXECUTABLE_NAME="daily-wallpapers-rust.exe"
mv "target/release/$EXECUTABLE_NAME" "$DWR_DIR"

# Check if the move was successful
if [ $? -ne 0 ]; then
    echo "Failed to move the executable!"
    exit 1
fi

echo "Executable moved to $DWR_DIR"

# Add .dwr/bin to the PATH permanently
SHELL_CONFIG="$HOME/.bashrc"
if [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
fi

echo "export PATH=\"$DWR_DIR:\$PATH\"" >> "$SHELL_CONFIG"

if [ $? -ne 0 ]; then
    echo "Failed to add $DWR_DIR to PATH!"
    exit 1
fi

echo "$DWR_DIR added to PATH permanently"

# Source the shell configuration to apply the changes
source "$SHELL_CONFIG"

# Exit the script
exit 0
