#!/bin/bash

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run with sudo" 1>&2
   exit 1
fi

# Name of the Rust project
PROJECT_NAME="motivate"

# Installation directory
INSTALLDIR="/opt/$PROJECT_NAME"

# Create motivate folder
mkdir -p $INSTALLDIR

# Copy the data folder and set permissions
cp -r data $INSTALLDIR/
chmod -R 777 $INSTALLDIR/data

# Copy the executable to the installation directory
cp ./target/release/$PROJECT_NAME $INSTALLDIR

# Check if the symbolic link already exists
if [ -L "/usr/local/bin/$PROJECT_NAME" ]; then
    # Remove existing symbolic link
    rm /usr/local/bin/$PROJECT_NAME
fi

# Create /usr/local/bin if it doesn't exist
mkdir -p /usr/local/bin

# Create a symbolic link to the installed binary
ln -s $INSTALLDIR/$PROJECT_NAME /usr/local/bin/$PROJECT_NAME

source ~/.bashrc

echo "$PROJECT_NAME installed successfully!"
