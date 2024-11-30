#!/bin/bash

# Check if the executables exist
if [ ! -f "target/release/multiclient-broadunicast" ]; then
    echo "Building project first..."
    ./build.sh
fi

# Run the main program
./target/release/multiclient-broadunicast 