# ARP Network Scanner 🌐🔍

## Overview 📋

ARP Network Scanner is a tool designed to scan local networks and identify devices by sending ARP (Address Resolution Protocol) requests. Built with Rust, it provides reliable network scanning with an easy-to-use interface.

## Features 🌟

- **Efficient Network Scanning**: Scans networks to identify devices efficiently.
- **Progress Tracking**: Displays a progress bar with a fun Pac-Man style to track the scanning progress.
- **Visual Feedback**: Uses styled console output to provide clear and colorful feedback.
- **Network Interface Selection**: Allows users to specify which network interface to use for scanning.

## Installation 🛠️

To use ARP Network Scanner, you need to have Rust installed on your system. If you don't have Rust, you can install it from [here](https://www.rust-lang.org/tools/install).

Clone the repository:
```
git clone https://github.com/Mario-SO/netscan-rs.git
cd netscan-rs
```

Build the project:
```
cargo build --release
```

## Usage 🚀

To run the ARP Network Scanner, use the following command:
```
cargo run --release <network_interface_name>
```

Replace `<network_interface_name>` with the name of the network interface you want to scan on. For example, `eth0` or `wlan0`.

See your available network interfaces using `arp -a` on *NIX systems

## Dependencies 📦

- `pnet` - Packet manipulation and network interfaces.
- `indicatif` - Progress bar handling.
- `console` - Styled console output.
- `ipnetwork` - Handling IP networks.

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check issues page.

## Final Thoughts 💭

Happy scanning! May your network exploration be fruitful and fun! 🎉👩‍💻👨‍💻

---

⌨️ with ❤️ by [mariodev](https://github.com/Mario-SO) 🚀

---