# rzip

`rzip` is a ZIP CLI focused on filename encoding control for cross-locale workflows.

It is designed for cases like:

- Archive created on a `zh_CN` Windows machine with WinRAR.
- Archive extracted on an `en_US` UTF-8 machine.
- Non-ASCII paths (CJK) become garbled without explicit encoding control.

`rzip` requires you to pass `--encoding` for both `pack` and `unpack`, so behavior stays explicit and predictable.

## Install

```bash
cargo install --locked rzip
```

## Usage

```bash
# Pack a directory with explicit filename encoding
rzip pack --input ./my-dir --output ./my-dir.zip --encoding gbk

# Unpack with explicit filename encoding
rzip unpack --input ./my-dir.zip --output ./out --encoding gbk
```

## Supported encodings

- `utf-8` (`utf8`, `unicode`)
- `gbk` (`cp936`, `936`)
- `shift_jis` (`shift-jis`, `sjis`, `cp932`)

## Why explicit encoding

Many archive tools infer filename encoding from locale or ZIP flags. In mixed environments this can produce corrupted paths during extraction. `rzip` makes the encoding choice explicit at runtime.
