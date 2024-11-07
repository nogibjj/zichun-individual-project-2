
# Rust CLI Binary with SQLite

## Overview
This project is a Command-Line Interface (CLI) tool developed in Rust, designed to interact with an SQLite database. It demonstrates full CRUD operations (Create, Read, Update, Delete) within an optimized Rust binary, packaged and built through GitLab Actions.

## Features
- **CRUD Operations:** Full support for managing data within an SQLite database.
- **Optimized Binary:** An optimized binary is created and stored as a downloadable artifact in GitLab Actions.
- **CI/CD Pipeline:** GitLab Actions handle building, linting, and testing for continuous integration.
- **GitHub Copilot Assistance:** GitHub Copilot was used to streamline development and improve code efficiency. (See details in the [Copilot Usage](#github-copilot-usage) section below.)

## Requirements
- **Rust** (latest stable version recommended)
- **SQLite** (for database handling)
- **Git** (for repository management)
- **GitLab Account** (for CI/CD pipeline configuration)

## Setup Instructions
1. **Clone the Repository:**
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Install Dependencies:**
   Ensure you have Rust and SQLite installed. To install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Compile and Run the Program:**
   ```bash
   cargo run --release
   ```

## Usage
The CLI tool provides the following commands:
- **Create**: Adds a new record to the SQLite database.
- **Read**: Retrieves records from the database.
- **Update**: Updates existing records.
- **Delete**: Deletes records from the database.

To use a specific command, run:
```bash
cargo run -- <command> [arguments]
```

### Examples
- **Creating a Record**:
  ```bash
  cargo run -- create "Sample Record" "Additional Data"
  ```
- **Reading Records**:
  ```bash
  cargo run -- read
  ```
- **Updating a Record**:
  ```bash
  cargo run -- update <record_id> "Updated Data"
  ```
- **Deleting a Record**:
  ```bash
  cargo run -- delete <record_id>
  ```

## GitHub Copilot Usage
GitHub Copilot provided intelligent code suggestions and helped optimize syntax. It assisted in:
- Generating Rust-specific syntax for error handling and type safety.
- Implementing initial CRUD logic and optimizing the database interaction flow.
- Developing and fine-tuning the CI/CD configuration with GitLab Actions.

## GitLab Actions CI/CD Pipeline
The CI/CD pipeline includes:
1. **Linting**: Ensures the code meets Rust formatting standards.
2. **Building**: Compiles the optimized Rust binary.
3. **Testing**: Runs automated tests on the CRUD operations to verify functionality.

To view the CI/CD pipeline status, refer to the [GitLab Actions section](https://gitlab.com/username/repo/-/pipelines) in this repository.

## Video Demo
Watch the [video demonstration](https://youtu.be/hbMcd7iyyCI) on YouTube, which provides a walkthrough of the project’s functionality and code explanation.
