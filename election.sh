#!/bin/bash
# If you are on linux I recommend copying this file to /usr/local/bin/ and renaming it to election
# sudo cp election.sh /usr/local/bin/election

# Exit immediately if a command exits with a non-zero status
set -e

# Navigate to the project directory (change this to the path of election.py)
cd /home/mate/Programozas/_Projects_/CaptchaLogin/cli

# Check if the 'env' directory exists
if [ ! -d "env" ]; then
    python -m venv env
    source env/bin/activate
    pip install --upgrade pip
    pip install -r requirements.txt
else
    source env/bin/activate
fi

# Run the Python script with provided arguments
python election.py "$@"

# Deactivate the virtual environment
deactivate