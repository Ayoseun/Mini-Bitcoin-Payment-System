# Mini-Bitpay: Rust-based Bitcoin Payment System

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.60.0-orange.svg)](https://www.rust-lang.org/)
[![Bitcoin](https://img.shields.io/badge/Bitcoin-sats-yellow.svg)](https://bitcoin.org/)

## Overview

Mini-Bitpay is a lightweight Rust-based payment system that accepts Bitcoin sats. It provides features for placing orders, viewing product lists, and managing transactions.

## Features

- **Place Order**: Allows users to place orders by selecting products and specifying units.
- **Show Product List**: Displays available products with their respective rates.
- **User-Friendly Interface**: Utilizes the `inquire` crate for an interactive and intuitive command-line interface.

## Prerequisites

- Rust 1.60.0 or later
- `inquire` crate (add to your `Cargo.toml`)

## Getting Started

### Installation

```bash
git clone https://github.com/[ayoseun]/mini-bitpay.git
cd mini-bitpay
cargo build --release
```
Usage
bash
Copy code
cargo run
Follow the on-screen prompts to place orders or view the product list.

Menu Options
Place Order: Select this option to place an order by choosing products and specifying units.
See Product List: View the list of available products with their rates.
Exit: Quit the mini-bitpay application.

## Contributing
Feel free to contribute by forking the repository, creating pull requests, or opening issues.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

 ## Acknowledgments
The inquire crate for providing an interactive CLI experience.

## Contact
For any questions or concerns, feel free to contact [Ayoseun]().