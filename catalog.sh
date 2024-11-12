#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# CHANGE THIS to the absolute path of the folder this script is in
project_path="/home/mate/Programozas/_Projects_/CaptchaLogin"

# Navigate to the project directory
cd $project_path

# Activate the virtual environment if it exists, otherwise create it first
if [ ! -d "env" ]; then
    echo "Creating virtual environment..."
    python -m venv env > /dev/null 2>&1
    source env/bin/activate > /dev/null 2>&1
    pip install --upgrade pip > /dev/null 2>&1
    pip install -r requirements.txt > /dev/null 2>&1
    echo "Virtual environment created."
else
    source env/bin/activate
fi

# Run the Python script with provided arguments
python catalog.py "$@"

# Deactivate the virtual environment
deactivate