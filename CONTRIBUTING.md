# Contributing to snag

Thank you for your interest in contributing to `snag`!

## Design Principles

Before opening a pull request, please keep `snag`'s core constraints in mind:

1. **Single-Purpose & Stateless**: `snag` reads standard input, matches line-by-line, and emits to standard output. It does not maintain disk state, background daemons, or network connections.
2. **Minimal Dependencies**: The binary relies on minimal dependencies (`regex` and `serde_json`). Avoid adding heavy dependencies or async runtimes.
3. **Stream Reliability**: All stream processing must gracefully handle invalid UTF-8 bytes (`String::from_utf8_lossy`) and broken pipe errors (`io::ErrorKind::BrokenPipe`).

## Development Workflow

1. **Fork and Clone**
   ```sh
   git clone https://github.com/programmersd21/snag.git
   cd snag
   ```

2. **Run Tests**
   ```sh
   cargo test
   ```

3. **Check Lints & Formatting**
   ```sh
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   ```

4. **Build Release**
   ```sh
   cargo build --release
   ```

## Pull Request Guidelines

- Ensure all existing unit and integration tests pass.
- Add test coverage in `tests/cli.rs` or unit tests for any new features or bug fixes.
- Keep commits descriptive and atomic.
