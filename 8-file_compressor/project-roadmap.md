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
- [x] Support basic flags:
  - [x] `-x/--extract` to indicate decompression
  - [x] `--zip` to indicate ZIP format
  - [x] `--gzip` to indicate GZIP format
  - [x] `--bzip2` to indicate BZIP2 format
- [x] Add basic logging and error handling via `anyhow`

## **Phase 2: Basic Archival and Extraction with ZIP**

**Goal**: Implement ZIP compression and extraction with the `zip` crate.

**Tasks**:

- [x] Add `zip` crate
- [x] Implement compression with the `--zip` flag:
  - [x] Support single input file
  - [x] Support multiple input files
  - [x] Support directory as input
  - [x] Output to specified directory
- [x] Implement extraction:
  - [x] Support single input archive
  - [x] Support multiple input archives
  - [x] Support archived directory as input
  - [x] Output to specified directory
- [x] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [x] Handle overwrite checks, file errors, etc
- [ ] Ensure preservation of file metadata such as ownership, permissions, timestamps, and symbolic links

## **Phase 3: Advanced Archival and Extraction with GZIP and BZIP2**

**Goal**: Implement GZIP and BZIP2 compression and extraction with the `flate2` and `bzip2` crates.

**Tasks**:

- [x] Add `flate2` crate
- [ ] Implement compression with the `--bzip2` and `--gzip` flags:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Implement extraction:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc
- [ ] Ensure preservation of file metadata such as ownership, permissions, timestamps, and symbolic links

## **Phase 4: Support for `.tar` + `.tar.gz` + `.tar.bz2`**

**Goal**: Layer in `tar` + optional compression (`gzip`, `bzip2`)

**Tasks**:

- [x] Add `tar` crate
- [ ] Implement compression with the `--tar`, `--tar-bzip2`, and `--tar-gzip` flags:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Implement extraction:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc
- [ ] Ensure preservation of file metadata such as ownership, permissions, timestamps, and symbolic links

## **Phase 5: Implement High-Performance Formats like `.zst` and `.xz`**

**Goal**: Add support for modern formats using `zstd` and `xz2`

**Tasks**:

- [x] Add `zstd` and `xz2` crates
- [ ] Implement compression with the `--tar-zst` and `--tar-xz2` flags:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Implement extraction:
  - [ ] Support single input file
  - [ ] Support multiple input files
  - [ ] Support directory as input
  - [ ] Output to specified directory
- [ ] Add test cases (**_Note_**: Consider using crates like `trycmd` and `snapbox`)
- [ ] Handle overwrite checks, file errors, etc
- [ ] Ensure preservation of file metadata such as ownership, permissions, timestamps, and symbolic links

## **Phase 6: Advanced Features and Improvements**

**Goal**: Implement advanced features such as archive encryption

**Tasks**:

- [ ] Add encryption and password protection for archives
- [ ] Add parallel processing support to speed up the processing of large files
- [ ] Add streaming support to process `stdin` and write to `stdout` (Integral for piping workflows)
- [ ] Add deduplication logic to save storage space

## **Phase 7: UX Features and Improvements**

**Goal**: Polish the CLI for aesthetics and ease-of-use

**Tasks**:

- [ ] Add `indicatif` crate for visual progress bars
- [ ] Add `--dry-run`, `--verbose`, and `--quiet` features and flags
- [ ] Improve error messages with `anyhow`

## **Phase 8: Packaging, Testing, and Distribution**

**Goal**: Prepare EZArchiver for real-world use

**Tasks**:

- [ ] Write unit and integration tests
- [ ] Add a README with usage examples
- [ ] Add version metadata and man page (Stretch goal)
- [ ] Package with `cargo build` plus relevant options
- [ ] Publish to crates.io (Stretch goal)
- [ ] Create cross-platform binaries using `cross` or `cargo-dist`
