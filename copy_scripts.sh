#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Copy the scripts to /usr/local/bin and make it executable
sudo cp catalog.sh /usr/local/bin/catalog
sudo cp election.sh /usr/local/bin/election

# Make the scripts executable
sudo chmod +x /usr/local/bin/catalog
sudo chmod +x /usr/local/bin/election

echo "Scripts copied and made executable"