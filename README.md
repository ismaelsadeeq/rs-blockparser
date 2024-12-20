### Rust Blockchain Parser with `libbitcoinkernel`

This is a simple blockchain block parser that utilizes **libbitcoinkernel** through the Rust wrapper available at:  
[**rust-bitcoinkernel**](https://github.com/TheCharlatan/rust-bitcoinkernel)

#### Installation Guide

**Requirements**

- Rust (ensure it is installed and properly set up)

**Steps**

1. Install the required dependencies (`rust-bitcoinkernel` and `bitcoin` crates):

    ```bash
    cargo build
    ```

2. Run the application:

    ```bash
    cargo run -- <dir> <start_height> <end_height>
    ```

This example demonstrates how to use the Rust wrapper for the `libbitcoinkernel` library to parse blockchain data from a specified height range.

You can easily adapt this to suit your specific requirements.


