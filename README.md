# Catalog & Election Login CLI Application

## Description
This is a simple CLI application that allows a user to login to [Catalog](https://catalog.inf.elte.hu/) or to [Election](https://election.inf.elte.hu/) with the captcha code.

## Requirements
- Rust with Cargo (tested on Rust 1.81.0)

## Installation
- Clone the repository
- Create a `.env` file and fill in the required fields based on `.env.example`
- Run `cargo build --release` to build the project (this will create the executables in `target/release/.` directory with the names `catalog` and `election`)

## Post Installation
- The .env file will be hard coded into the executables, so you can delete it after the executables are built (optional)
- Add these executables to your PATH so you can run them from anywhere in the terminal (optional)

## Usage
You can run the following commands from the terminal:
- `catalog <captcha_code>` to login to Catalog
- `election <captcha_code>` to login to Election