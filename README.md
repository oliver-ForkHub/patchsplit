# patchsplit

Language: English | [Simplified Chinese](README_zh-cn.md)

`patchsplit` is a Rust CLI that downloads a GitHub Pull Request `.patch` file
and splits it into one patch file per commit.

## Usage

```sh
patchsplit <owner/repo> <pr-number> [--out <dir>] [--force] [--squash]
patchsplit <owner> <repo> <pr-number> [--out <dir>] [--force] [--squash]
```

Examples:

```sh
patchsplit rust-lang/rust 12345
patchsplit openai codex 42 -o pr-42-patches
```

The default output directory is `patches/`. Output files are named with a
four-digit index and the commit subject:

```text
patches/
  0001-add-parser.patch
  0002-wire-cli.patch
```

Existing output files are not overwritten by default. Pass `--force` to replace
them.

### Aggregate all commits

```sh
patchsplit openai/codex 42 --squash -o pr-42-patches
git apply pr-42-patches/pr-42.patch
```

`-s, --squash` downloads GitHub's aggregate PR `.diff` and writes a single
`pr-<pr-number>.patch`. This represents the net change from the PR's merge base
to its head: repeated edits are combined and reverted changes disappear.
It does not concatenate per-commit patches. The output is a raw diff for
`git apply` on the corresponding base, without individual commit messages or
authorship (it is not a `git am` mailbox). An empty net diff is reported as an
empty patch and no file is written. Binary changes are limited to the data
GitHub includes in its diff; binary file contents may not be included.

## Options

- `-o, --out <dir>`: Output directory for patch files.
- `-f, --force`: Overwrite existing patch files.
- `-s, --squash`: Write the PR's net diff as one patch.
- `-h, --help`: Show help.
- `-V, --version`: Show version.

## Dependencies

The CLI uses `thiserror` for internal error types. It calls the system `curl`
command to download GitHub `.patch` files, so `curl` must be available in
`PATH` at runtime.

## Localization

User-facing CLI text is wired through a PO-based i18n layer. At runtime,
`patchsplit` picks the first locale from `PATCHSPLIT_LANGUAGE`, `LANGUAGE`,
`LC_ALL`, `LC_MESSAGES`, or `LANG`, and reads UTF-8 `.po` catalogs from
`PATCHSPLIT_LOCALEDIR` or installation-relative locations next to the
executable.

Refresh the translation template with GNU gettext tools:

```sh
scripts/update-pot.sh
```

The template is generated at `po/patchsplit.pot`. Source files used for
extraction are listed in `po/POTFILES.in`.

## Build

Tests (`cargo test`) also require Git on Unix to verify that aggregate patches
apply to the expected file tree.

```sh
cargo build --release
```

The release binary is generated at:

```text
target/release/patchsplit
```

## Release

Pushing a `v*` tag triggers GitHub Actions to build release packages for three
platforms and automatically create a GitHub draft release:

```sh
git tag v1.x.x
git push origin v1.x.x
```

You can also run the `Release` workflow manually from GitHub Actions. Select the
branch or commit to package, then enter the release tag. If the tag does not
exist, it will point to the workflow commit. The workflow creates these files:

- `patchsplit-linux-x86_64.tar.gz`
- `patchsplit-macos-x86_64.tar.gz`
- `patchsplit-windows-x86_64.zip`
- `patchsplit_<version>_amd64.deb`
- `patchsplit-<version>-<release>.*.x86_64.rpm`

Releases are created as drafts, so they should be reviewed and published from
the GitHub Releases page.

### Launchpad PPA

Publishing a GitHub release triggers the `Notify PPA` workflow. It builds and
signs a Debian source package, then uploads it to Launchpad so the configured
PPA starts building the new version. Configure these repository variables:

- `PPA_OWNER`: Launchpad account name.
- `PPA_NAME`: PPA name, without the `ppa:` prefix.
- `PPA_GPG_KEY_ID`: full fingerprint of the primary signing key (not a signing subkey ID).
- `PPA_MAINTAINER_NAME` and `PPA_MAINTAINER_EMAIL`: optional source package metadata.

Store the ASCII-armored private key as the `PPA_GPG_PRIVATE_KEY` repository
secret. The workflow can also be run manually with a tag, branch, or commit in
the `ref` input.

## Install

### Linux

On Debian and Ubuntu, download the `.deb` release asset and install it with:

```sh
sudo apt install ./patchsplit_<version>_amd64.deb
```

On Fedora, RHEL, and compatible distributions, download the `.rpm` release
asset and install it with:

```sh
sudo dnf install ./patchsplit-*.x86_64.rpm
```

If the release also includes a debuginfo package, install the main package
`patchsplit-<version>-<release>.x86_64.rpm` instead of the debuginfo package.

> [!NOTE]
> For Arch Linux, use the [`patchsplit-bin`](https://aur.archlinux.org/packages/patchsplit-bin) AUR package
> maintained by [lingbopro](https://github.com/lingbopro).
>
> - Using `paru`: `paru -S patchsplit-bin`
> - Using `yay`: `yay -S patchsplit-bin`

```sh
tar -xzf patchsplit-linux-x86_64.tar.gz
chmod +x patchsplit
sudo install -m 755 patchsplit /usr/local/bin/patchsplit
patchsplit --version
```

### macOS

```sh
tar -xzf patchsplit-macos-x86_64.tar.gz
chmod +x patchsplit
sudo install -m 755 patchsplit /usr/local/bin/patchsplit
patchsplit --version
```

If macOS blocks the downloaded binary, remove the quarantine attribute:

```sh
xattr -d com.apple.quarantine /usr/local/bin/patchsplit
```

### Windows

Extract the archive in PowerShell:

```powershell
Expand-Archive .\patchsplit-windows-x86_64.zip -DestinationPath .\patchsplit
.\patchsplit\patchsplit.exe --version
```

To use it globally, add the extracted `patchsplit` directory to your user `Path`
environment variable.
