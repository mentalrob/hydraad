# HydraAD

🚧 **Work in Progress** 🚧

HydraAD is a command-line tool for Active Directory penetration testing and red team operations. This project is currently under active development.

## Features (Planned/In Development)

- **Domain Controller Management**
  - Add and configure domain controllers
  - List available domain controllers
  - Switch between different domain controllers

- **Credential Management**
  - Store and manage credentials (passwords, NTLM hashes, tokens)
  - List and filter credentials by domain, type, or validation status
  - Remove credentials with safety confirmations

- **Interactive CLI**
  - Tab completion for commands
  - Context-aware prompts showing current domain controller
  - Command history and search

## Installation

This project is not yet ready for general use. To build from source:

```bash
git clone <repository-url>
cd hydraad
cargo build --release
```

## Usage

```bash
# Start the interactive shell
./target/release/hydraad

# Domain controller operations
dc add <ip> [--ldaps] [--ldap-port <port>]
dc list
dc use <domain-name>

# Credential operations
creds add <username> <domain> <auth-data> [options]
creds list [--domain <domain>] [--validated-only]
creds remove <credential-id> [--force]
```

## Development Status

This project is in early development. Features may be incomplete, unstable, or subject to breaking changes.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License

Copyright (c) 2024 HydraAD Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Disclaimer

This tool is intended for authorized penetration testing and educational purposes only. Users are responsible for ensuring they have proper authorization before using this tool against any systems.
