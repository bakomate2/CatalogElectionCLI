# Catalog & Election Login CLI Application

## Description
This is a simple CLI application that allows a user to login to [Catalog](https://catalog.inf.elte.hu/) or to [Election](https://election.inf.elte.hu/) with the captcha code.

## Requirements
- Linux based OS (tested on Arch Linux)
- Python 3.12.7 (might work with other versions as well but not tested)
- `virtualenv` pip package (to create a virtual environment)

## Before you start:
- Clone the repository
- Based on `.env.example` create a `.env` file and fill in the required fields
- Run `chmod +x copy_script.sh` to make the script executable
- Run `./copy_script.sh`

## Usage
<u>You can run the following commands from anywhere in the terminal:</u>
- Run `catalog <captcha_code>` to login to Catalog
- Run `election <captcha_code>` to login to Election