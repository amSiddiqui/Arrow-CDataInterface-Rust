# Arrow Exporter Example in Rust

This project demonstrates how to use the Apache [Arrow C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html) in Rust, C, and Python. It is an example adapted from Matt Topol's book on [Arrow](https://www.amazon.co.uk/Memory-Analytics-Apache-Arrow-hierarchical-ebook/dp/B09X76LNN9), translated into Rust.

## Overview

The project consists of:

- **Rust Library**: A Rust implementation that generates random `int32` data and exposes it via the Arrow C Data Interface.
- **C Program**: A C program that uses the Rust library to obtain the data and prints the first 10 elements.
- **Python Script**: A Python script that loads the Rust library and imports the data into a PyArrow `Array`.

## Contents

- `arrow-exporter/`: The Rust library project.
  - `Cargo.toml`
  - `src/`
    - `lib.rs`
  - `main.c`: The C program using the Rust library.
  - `example.py`: The Python script using the Rust library.

## Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **C Compiler**: `gcc` or `clang` for compiling the C program.
- **Python 3**: Install Python 3 from [python.org](https://www.python.org/).
- **Python Packages**:
  - `pyarrow`
  - `cffi`

  Install them using:

  ```bash
  pip install pyarrow cffi
  ```

## Building and Running the Project

### 1. Build the Rust Library

Navigate to the `arrow-exporter/` directory and build the library:

```bash
cd arrow-exporter
cargo build --release
```

This will generate a shared library:

- On **macOS**: `target/release/libarrow_exporter.dylib`
- On **Linux**: `target/release/libarrow_exporter.so`

### 2. Run the C Example

#### a. Compile the C Program

The `main.c` file defines the `ArrowArray` struct directly and does not include `abi.h`.

Ensure you are in the `arrow-exporter/` directory:

```bash
cd arrow-exporter
```

##### On macOS:

```bash
gcc main.c -o main \
    -L ./target/release \
    -larrow_exporter \
    -Wl,-rpath,@loader_path/target/release
```

##### On Linux:

```bash
gcc main.c -o main \
    -L ./target/release \
    -larrow_exporter \
    -Wl,-rpath,$ORIGIN/target/release
```

- The `-L` flag specifies the directory of the Rust library.
- The `-larrow_exporter` flag links against the Rust library.
- The `-Wl,-rpath,...` flag sets the runtime library search path.
- Since `abi.h` is not included and `ArrowArray` is defined within `main.c`, there is no need for the `-I` flag.

#### b. Run the C Program

```bash
./main
```

You should see output similar to:

```
data[0] = 123456789
data[1] = -987654321
...
```

### 3. Run the Python Example

Ensure you are in the `arrow-exporter/` directory:

```bash
cd arrow-exporter
```

#### a. Adjust the Python Script if Necessary

The `example.py` script uses the Rust library. If needed, adjust the library path in the script to point to the Rust library:

```python
lib = ffi.dlopen("./target/release/libarrow_exporter.dylib")
```

- On Linux, change `.dylib` to `.so`:

  ```python
  lib = ffi.dlopen("./target/release/libarrow_exporter.so")
  ```

#### b. Run the Python Script

```bash
python example.py
```

You should see output similar to:

```
<pyarrow.lib.Int32Array object at 0x7f8b6c8b8b80>
[
  123456789,
  -987654321,
  ...
]
```

## Project Structure

```
arrow-exporter/
├── Cargo.toml
├── src/
│   └── lib.rs
├── main.c
└── example.py
```

## Notes

- **Shared Library Extension**: On macOS, shared libraries have the `.dylib` extension, while on Linux, they use `.so`. Ensure you use the correct extension for your platform when loading the library.
- **Library Path Adjustments**: If you move the directories or run the scripts from different locations, update the paths to the Rust library accordingly.
- **Dependencies**: The Python script requires `pyarrow` and `cffi`. Install them using `pip install pyarrow cffi`.

- **Python Script**: The `example.py` script uses `pyarrow` and `cffi` to interface with the Rust library and import the data into a PyArrow `Array`.

## References

- **Apache Arrow C Data Interface**: [Arrow C Data Interface Documentation](https://arrow.apache.org/docs/format/CDataInterface.html)
- **Matt Topol's Book on Arrow**: This example is adapted from Matt Topol's [book](https://www.amazon.co.uk/Memory-Analytics-Apache-Arrow-hierarchical-ebook/dp/B09X76LNN9), demonstrating practical usage of Arrow across different languages.

## License

This project is licensed under the MIT License.