# **EZArchiver**
A command-line file archival and extraction utility written in Rust.

#### **Usage**

- `ezarc [OPTIONS] <input> <output>`
    - _Options_:
        - `-x` to extract archived file. **Archives by default**
        - Flags for different compression schemes
        - `-d` to accept directories. **Accepts files by default (Stretch goal)**
        - Accept multiple input and output files **(Stretch goal)**
        - Output input file(s) into specified output directory **(Stretch goal)**

#### **Crates**

- `clap`: For parsing command-line arguments
- `anyhow`: For smarter error handling
- `zip`: For reading and writing ZIP archives
- `tar`: For TAR archive manipulation
- `flate2`: For Gzip compression/decompression
- `indicatif`: For terminal-based progress bar

#### **Project Structure**

```rs
file_compressor/
|-- src/
|   |-- main.rs             // CLI parsing
|   |-- utils.rs            // Common utility functions
|   |-- commands/
|   |   |-- compress.rs     // Compression logic
|   |   |-- extract.rs      // Extraction logic
|-- Cargo.toml
|-- Cargo.lock
```

## **Phase 1: Project Initialization and CLI Parsing**
**Goal**: Set up a new Rust project with argument parsing using `clap`.

**Tasks**:

- [x] Create a new binary crate
- [x] Add necessary dependencies: `clap` and `anyhow`
- [x] Create a modular project structure to support the `extract` and `compress` commands as well as any future commands
- [x] Define CLI structure with subcommands:
    - [x] `compress` - Archive and compress files
    - [x] `extract` - Extract files from archives
- [ ] Support basic flags:
    - [x] `-o/--output` to indicate output file location
    - [ ] `-d/--directory` to indicate input and/or output directory
    - [x] `-x/--extract` to indicate decompression
- [x] Add basic logging and error handling via `anyhow`

## **Phase 2: Basic Archival and Extraction with ZIP**
**Goal**: Implement ZIP compression and extraction with the `zip` crate.

**Tasks**:

- [x] Add `zip` crate
- [x] Implement compression with the `--zip` flag:
    - [x] Single input file into single output `.zip` file
    - [x] Multiple input files into multiple output `.zip` files
    - [ ] Archive a specified directory with the `-d` flag
    - [x] Archive _**to**_ a specified directory with the `-d` flag
- [ ] Implement extraction:
    - [ ] Single input `.zip` file into single output file
    - [ ] Multiple input `.zip` files into multiple output files
    - [ ] Extract a specified directory with the `-d` flag
    - [ ] Extract _**to**_ a specified directory with the `-d` flag
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc

## **Phase 3: Advanced Archival and Extraction with GZIP and BZIP2**
**Goal**: Implement GZIP and BZIP2 compression and extraction with the `flate2` and `bzip2` crates.

**Tasks**:

- [x] Add `flate2` crate
- [ ] Implement compression with the `--bzip2` and `--gzip` flags:
    - [ ] Single input file into single output `.bz2` or `.gz` file
    - [ ] Multiple input files into multiple output `.bz2` or `.gz` files
    - [ ] Archive a specified directory with the `-d` flag
    - [ ] Archive _**to**_ a specified directory with the `-d` flag
- [ ] Implement extraction:
    - [ ] Single input `.bz2` or `.gz` file into single output file
    - [ ] Multiple input `.bz2` or `.gz` files into multiple output files
    - [ ] Extract a specified directory with the `-d` flag
    - [ ] Extract _**to**_ a specified directory with the `-d` flag
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc

## **Phase 4: Support for `.tar` + `.tar.gz` + `.tar.bz2`**
**Goal**: Layer in `tar` + optional compression (`gzip`, `bzip2`)

**Tasks**:

- [x] Add `tar` crate
- [ ] Implement compression with the `--tar`, `--tar-bzip2`, and `--tar-gzip` flags:
    - [ ] Single input file into single output `.tar`, `.tar.bz2` or `.tar.gz` file
    - [ ] Multiple input files into multiple output `.tar`, `.tar.bz2` or `.tar.gz` files
    - [ ] Archive a specified directory with the `-d` flag
    - [ ] Archive _**to**_ a specified directory with the `-d` flag
- [ ] Implement extraction:
    - [ ] Single input `.tar`, `.tar.bz2` or `.tar.gz` file into single output file
    - [ ] Multiple input `.tar`, `.tar.bz2` or `.tar.gz` files into multiple output files
    - [ ] Extract a specified directory with the `-d` flag
    - [ ] Extract _**to**_ a specified directory with the `-d` flag
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc

## **Phase 5: Implement High-Performance Formats like `.zst` and `.xz`**
**Goal**: Add support for modern formats using `zstd` and `xz2`

**Tasks**:

- [x] Add `zstd` and `xz2` crates
- [ ] Implement compression with the `--tar-zst` and `--tar-xz2` flags:
    - [ ] Single input file into single output `.tar.zst` or `.tar.xz` file
    - [ ] Multiple input files into multiple output `.tar.zst` or `.tar.xz` files
    - [ ] Archive a specified directory with the `-d` flag
    - [ ] Archive _**to**_ a specified directory with the `-d` flag
- [ ] Implement extraction:
    - [ ] Single input `.tar.zst` or `.tar.xz` file into single output file
    - [ ] Multiple input `.tar.zst` or `.tar.xz` files into multiple output files
    - [ ] Extract a specified directory with the `-d` flag
    - [ ] Extract _**to**_ a specified directory with the `-d` flag
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc

## **Phase 6: UX Features and Improvements**
**Goal**: Polish the CLI for aesthetics and ease-of-use

**Tasks**:

- [ ] Add `indicatif` crate for visual progress bars
- [ ] Add support for reading from `stdin` and writing to `stdout` with appropriate flags
- [ ] Add `--dry-run`, `--verbose`, and `--quiet` features and flags
- [ ] Improve error messages with `anyhow`

## **Phase 7: Packaging, Testing, and Distribution**
**Goal**: Prepare EZArchiver for real-world use

**Tasks**:

- [ ] Write unit and integration tests
- [ ] Add a README with usage examples
- [ ] Add version metadata and man page (Stretch goal)
- [ ] Package with `cargo build` plus relevant options
- [ ] Publish to crates.io (Stretch goal)
- [ ] Create cross-platform binaries using `cross` or `cargo-dist`