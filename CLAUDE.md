# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build (default features include audio)
cargo build

# Build without audio feature (smaller binary, no system audio deps)
cargo build --no-default-features

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings -Z unstable-options

# Run the application
cargo run -- <word>          # translate a word
cargo run -- 你好             # works with Chinese input
cargo run                    # interactive mode (REPL)
```

## Project Overview

`clitrans` is a command-line dictionary/translator that scrapes online translation services (Bing Dictionary, Youdao) and displays results in the terminal. It requires no API tokens.

## Architecture

### Translation Flow

1. **CLI parsing** (`src/cli.rs`) - Uses clap with derive macros. Engines are specified via `-e` flag or `CLITRANS_ENGINES` env var.

2. **Concurrent engine queries** (`src/main.rs:translate`) - Spawns threads for each engine, returns the first successful result. Engines race; fastest response wins.

3. **Engine implementations** (`src/engine/`) - Each engine (bing, youdao) implements the `Translate` trait. They:
   - Make HTTP requests via `ureq`
   - Parse HTML responses using `scraper` crate
   - Return a `Translation` struct

4. **Output rendering** (`src/translation.rs`) - Colored terminal output with pronunciations, explanations by part of speech, and web phrases.

### Key Types

- `Translation` - Core result type containing query, URL, pronunciations, explanations, and phrases
- `Translate` trait - Interface for translation engines (`fn translate(&self, text: &str) -> Result<Option<Translation>>`)
- `Engine` enum - Available engines (Youdao, Bing)

### Build System

The `build.rs` script:
- Embeds git commit info and rustc version into the binary
- Generates test code from `src/engine/*/test_data.json` files using the `quote` crate

### Features

- `audio` (default) - Enables pronunciation playback via `rodio`. Requires system audio libraries (libasound2 on Linux).
- `full` - Alias for audio

## Testing

Tests are generated at build time from JSON fixtures in `src/engine/{bing,youdao}/test_data.json`. The generated test files (`test.rs`) are marked read-only and should not be edited directly.

To add a new test case, add an entry to the appropriate `test_data.json` file.
