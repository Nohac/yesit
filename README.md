
# yesit

`yesit` is an enhanced version of classic Unix `yes` command. While retaining
the simplicity of `yes`, `yesit` extends its functionality to support
interactive applications by automatically responding to prompts.

## Installation

1. Ensure you have Rust installed on your system.
2. Clone the `yesit` repository:
   ```sh
   git clone https://github.com/Nohac/yesit
   ```
3. Navigate into the `yesit` directory:
   ```sh
   cd yesit
   ```
4. Build the project using Cargo:
   ```sh
   cargo build --release
   ```
5. The executable will be located in `target/release/`.
6. It's also possible to install `yesit` by running:
   ```sh
   cargo install --path .
   ```

## Usage

```sh
yesit [OPTIONS] -- [COMMAND]
```

### Example

Automatically agree to all prompts with "yes" every second for a script:

```sh
yesit -p yes -i 1s -- ./your_script.sh
```

## License

`yesit` is open-source software licensed under the MIT license. See the LICENSE file for more details.

---

Disclaimer: This README has been generated by an AI based on the provided program information.
