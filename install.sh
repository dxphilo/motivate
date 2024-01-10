#!/bin/bash

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run with sudo" 1>&2
   exit 1
fi

# Name of the Rust project
PROJECT_NAME="motivate"

# Installation directory
INSTALLDIR="/opt/$PROJECT_NAME"

# Create the installation folder
mkdir -p $INSTALLDIR

# Move to the project directory
cd $INSTALLDIR

# Clone the repository
git clone https://github.com/dxphilo/motivate.git

# Move to the project directory
cd motivate

# Install the project using cargo
cargo install --path .

# Create a symbolic link to the installed binary
ln -s $INSTALLDIR/target/release/$PROJECT_NAME /usr/local/bin/$PROJECT_NAME

# Optional: Copy data folder and set permissions
# cp -r $INSTALLDIR/data $INSTALLDIR
# chmod -R 777 $INSTALLDIR/data

echo "$PROJECT_NAME installed successfully!"