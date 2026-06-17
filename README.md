# rzip

`rzip` is a ZIP CLI focused on filename encoding control for cross-locale workflows.

It is designed for cases like:

- Archive created on a `zh_CN` Windows machine with WinRAR.
- Archive extracted on an `en_US` UTF-8 machine.
- Non-ASCII paths (CJK) become garbled without explicit encoding control.

`rzip` defaults to `utf-8` when `--encoding` is omitted, and you can still override it with `-e/--encoding` for cross-locale archives.

## Install

```bash
cargo install --locked rzip
```

## Usage

```bash
# Pack a directory with explicit filename encoding
rzip pack -i ./my-dir -o ./my-dir.zip -e gbk -r

# Unpack with explicit filename encoding
rzip unpack -i ./my-dir.zip -o ./out -e gbk

# Omit encoding (defaults to utf-8)
rzip pack -i ./my-dir -o ./my-dir.zip -r
```

## Common options

- `-r, --recursive` (pack only): include nested files/directories.
- `--include <GLOB>`: only process matching paths. Repeatable.
- `--exclude <GLOB>`: skip matching paths. Repeatable.

Example:

```bash
rzip pack -i ./project -o ./project.zip -e utf-8 -r \
  --exclude ".git/**" \
  --exclude "target/**" \
  --include "**/*.rs"
```

## Supported encodings

- `utf-8` (`utf8`, `unicode`)
- `gbk` (`cp936`, `936`)
- `shift_jis` (`shift-jis`, `sjis`, `cp932`)

## Why explicit encoding

Many archive tools infer filename encoding from locale or ZIP flags. In mixed environments this can produce corrupted paths during extraction. `rzip` makes the encoding choice explicit at runtime.
