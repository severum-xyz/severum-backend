# Severum Backend

## Overview

**Severum Backend** is the backend component of the Severum Web3 security platform. Written in Rust, it provides the foundation for managing security challenges, fetching them from private repositories, and integrating with the Severum Sandbox to execute smart contract tests. Severum Core will ensure smooth communication between the frontend and backend, track user progress, and provide real-time reports.

## Features

- **Challenge Management:** Handles the retrieval, organization, and management of Web3 security challenges.
- **Integration with Severum Sandbox:** Coordinates with the Severum Sandbox to test and simulate Solidity smart contract exploits.
- **Real-Time Reports:** Tracks user progress and provides reports on the completion and outcomes of challenges.
- **Backend Communication:** Ensures smooth interactions between frontend and backend, enabling seamless user experiences.

## Installation

To get started with Severum Core, follow these steps:

### 1. Clone the repository
```bash
git clone https://github.com/severum-xyz/severum-backend.git
cd severum-backend
```

### 2. Install Dependencies

Make sure you have Rust installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).

Once Rust is installed, you can set up your environment by running:

```bash
cargo build
```

### 3. Running the project

To run Severum Core locally for development:

```bash
cargo run
```

The server will start and listen for requests. You can access it via `localhost:3000`.

## Contributing

We welcome contributions! Please fork the repository, make your changes, and submit a pull request. Before contributing, please ensure that:

- You have reviewed the project's contribution guidelines (to be added later).
- Your changes follow the coding style and quality guidelines.
- You have written tests for your changes where applicable.

### License

Severum Core is open for contributions under the [MIT License](LICENCE).

## Contact

For questions or feedback, please reach out to the Severum team at **contact@severum.xyz**.
