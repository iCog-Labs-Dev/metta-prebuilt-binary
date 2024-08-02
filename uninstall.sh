#!/bin/bash

# Function to print error messages
error() {
    echo "$1" >&2
    exit 1
}

# Variables
INSTALL_DIR="$HOME/metta-bin"
VENV_DIR="$INSTALL_DIR/venv"
DESTINATION_PATH="/usr/local/bin/metta"
WRAPPER_PATH="/usr/local/bin/metta-run"

# Step 1: Remove the binary from /usr/local/bin
if [ -f "$DESTINATION_PATH" ]; then
    echo "Removing the binary from $DESTINATION_PATH..."
    sudo rm $DESTINATION_PATH || error "Failed to remove the binary from /usr/local/bin."
else
    echo "Binary $DESTINATION_PATH does not exist. Skipping."
fi

# Step 2: Remove the wrapper script from /usr/local/bin
if [ -f "$WRAPPER_PATH" ]; then
    echo "Removing the wrapper script from $WRAPPER_PATH..."
    sudo rm $WRAPPER_PATH || error "Failed to remove the wrapper script from /usr/local/bin."
else
    echo "Wrapper script $WRAPPER_PATH does not exist. Skipping."
fi

# Step 3: Remove the backup wrapper script if it exists
if [ -f "${WRAPPER_PATH}.bak" ]; then
    echo "Removing the backup wrapper script from ${WRAPPER_PATH}.bak..."
    sudo rm "${WRAPPER_PATH}.bak" || error "Failed to remove the backup wrapper script from /usr/local/bin."
else
    echo "Backup wrapper script ${WRAPPER_PATH}.bak does not exist. Skipping."
fi

# Step 4: Deactivate and remove the Python virtual environment
if [ -d "$VENV_DIR" ]; then
    echo "Deactivating and removing the Python virtual environment..."
    deactivate 2>/dev/null || true
    rm -rf $VENV_DIR || error "Failed to remove the Python virtual environment."
else
    echo "Virtual environment $VENV_DIR does not exist. Skipping."
fi

# Step 5: Remove the cloned repository
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing the cloned repository directory $INSTALL_DIR..."
    rm -rf $INSTALL_DIR || error "Failed to remove the cloned repository directory."
else
    echo "Directory $INSTALL_DIR does not exist. Skipping."
fi

echo "Uninstallation complete!"
