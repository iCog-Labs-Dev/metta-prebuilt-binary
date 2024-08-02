#!/bin/bash

# Function to print error messages
error() {
    echo "$1" >&2
    exit 1
}

# Variables
REPO_URL="https://github.com/iCog-Labs-Dev/metta-prebuilt-binary.git"
INSTALL_DIR="$HOME/metta-bin"
VENV_DIR="$INSTALL_DIR/venv"
BINARY_PATH="$INSTALL_DIR/v0.1.11/metta"
DESTINATION_PATH="/usr/local/bin/metta"
WRAPPER_PATH="/usr/local/bin/metta-run"

# Step 1: Update the repository
if [ -d "$INSTALL_DIR" ]; then
    echo "Updating the repository..."
    cd $INSTALL_DIR || error "Failed to enter the repository directory."
    git pull $REPO_URL || error "Failed to pull latest changes from the repository."
else
    echo "The repository does not exist. Cloning afresh..."
    git clone $REPO_URL $INSTALL_DIR || error "Failed to clone repository."
    cd $INSTALL_DIR || error "Failed to enter the repository directory."
fi

# Step 2: Update Python virtual environment
if [ -d "$VENV_DIR" ]; then
    echo "Updating Python virtual environment..."
    source $VENV_DIR/bin/activate || error "Failed to activate Python virtual environment."
    if [ -f "./requirements.txt" ]; then
        pip install --upgrade -r requirements.txt || error "Failed to update Python dependencies."
    else
        echo "No requirements.txt found. Skipping Python dependencies update."
    fi
else
    echo "Virtual environment does not exist. Creating anew..."
    python3 -m venv $VENV_DIR || error "Failed to create Python virtual environment."
    source $VENV_DIR/bin/activate || error "Failed to activate Python virtual environment."
    if [ -f "./requirements.txt" ]; then
        pip install -r requirements.txt || error "Failed to install Python dependencies."
    else
        error "No requirements.txt found. Failed to install Python dependencies."
    fi
fi

# Step 3: Update the binary in /usr/local/bin
if [ -f "$BINARY_PATH" ]; then
    echo "Updating the binary in $DESTINATION_PATH..."
    sudo cp $BINARY_PATH $DESTINATION_PATH || error "Failed to update the binary in /usr/local/bin."
    sudo chmod +x $DESTINATION_PATH || error "Failed to make the binary executable."
else
    error "Binary not found. Update failed."
fi

# Step 4: Update the wrapper script
if [ -f "$WRAPPER_PATH" ]; then
    echo "Updating the wrapper script at $WRAPPER_PATH..."
    sudo rm "$WRAPPER_PATH" || error "Failed to remove old wrapper script."
fi

echo "#!/bin/bash" | sudo tee $WRAPPER_PATH
echo "source $VENV_DIR/bin/activate && metta \"\$@\"" | sudo tee -a $WRAPPER_PATH
sudo chmod +x $WRAPPER_PATH || error "Failed to create the wrapper script."

echo "Update complete! You can now run 'metta' from any path."
echo "To use the metta with python environment automatically activated, run 'metta-run' instead of 'metta'."
echo "To activate the virtual environment manually, run 'source $VENV_DIR/bin/activate' then use 'metta' as usual."
echo "To deactivate it, simply run 'deactivate'."
