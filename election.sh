#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Get the absolute path of the folder this script is in
project_path = $(cd "$(dirname "$0")" && pwd)

# Navigate to the project directory
cd $project_path

# Activate the virtual environment if it exists, otherwise create it first
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