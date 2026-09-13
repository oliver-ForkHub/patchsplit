# Contributing to patchsplit

Thank you for your interest in contributing to patchsplit!

Whether you are fixing a bug, improving documentation, or adding a new feature, your contributions are welcome.

## Development environment

patchsplit is written in Rust.

Before starting development, make sure you have:

* [Rust](https://www.rust-lang.org/tools/install) installed.
* Git installed.
* A working development environment for your platform.

Clone the repository:

```bash
git clone https://github.com/zitzhen/patchsplit.git
cd patchsplit
```

## Building

Build patchsplit in debug mode:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

## Testing and checks

Before submitting a pull request, please run the relevant checks:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

If a check is not applicable to your change, explain why in your pull request.

## Making changes

1. Create a new branch for your change.
2. Make your changes.
3. Run the relevant tests and checks.
4. Commit your changes with a clear commit message.
5. Open a pull request.

Please keep changes focused and avoid unrelated modifications.

## Pull requests

When opening a pull request, please include:

* A clear description of what changed.
* The reason for the change.
* Testing performed.
* Any relevant screenshots or command output, if applicable.

For bug fixes, please include steps to reproduce the issue when possible.

## Reporting bugs

Please use [GitHub Issues](https://github.com/zitzhen/patchsplit/issues) to report bugs.

Include:

* patchsplit version.
* Operating system and architecture.
* Steps to reproduce.
* Expected behavior.
* Actual behavior.
* Relevant error messages or logs.

Please avoid including sensitive information in bug reports.

## Code style

* Follow the existing Rust code style.
* Run `cargo fmt` before submitting changes.
* Prefer clear, maintainable code.
* Add or update tests when appropriate.
* Keep documentation up to date when behavior changes.

## Packaging

Packaging-related changes should include the relevant packaging files and any required build or validation steps.

If your change affects Debian, RPM, or other distribution packages, please explain the impact in your pull request.

## License

By contributing to patchsplit, you agree that your contributions will be licensed under the same license as the project, as specified in the repository's `LICENSE` file.

Thank you for helping improve patchsplit!
