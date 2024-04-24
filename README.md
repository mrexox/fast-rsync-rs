# fast-rsync-rs

This is a wrapper for a Rust crate [`fast_rsync`](https://crates.io/crates/fast_rsync).

## Install

```bash
npm install fast-rsync-rs
```

## Usage

Calculate a diff between two files.

```typescript
import { readFileSync } from 'node:fs'
import { diffFiles, patchFile } from 'fast-rsync-rs'

const delta = diffFiles('path/to/file/old.bin', 'path/to/file/new.bin')

// Apply the delta
const result: Buffer = patchFile('path/to/file/old.bin', delta)

// The contents must be similar. However in some cases the diffs are not applicable. See https://docs.rs/fast_rsync/0.2.0/fast_rsync/fn.diff.html#security
assert(Buffer.compare(result, readFileSync('path/to/file/new.bin')) === 0)
```

Use the signature buffer for faster diffing against multiple files.

```typescript
import { readFileSync } from 'node:fs'
import { diff, fileSignature, apply } from 'fast-rsync-rs'

const signature = fileSignature('version0.bin')

const buf1 = readFileSync('version1.bin')
const buf2 = readFileSync('version2.bin')
const buf3 = readFileSync('version3.bin')

const delta1 = diff(signature, buf1)
const delta2 = diff(signature, buf2)
const delta3 = diff(signature, buf3)

const version2 = apply(readFileSync('version0.bin'), delta2)

assert(Buffer.compare(version2, buf2) === 0)
```

Use custom signature options.

```typescript
import { readFileSync } from 'node:fs'
import { diff, signature, apply } from 'fast-rsync-rs'

const buf0 = readFileSync('version0.bin')
const buf1 = readFileSync('version1.bin')

const sig = signature(buf0, { bufferSize: 512, cryptoHashSize: 8 })

const delta = diff(sig, buf1)

assert(Buffer.compare(buf1, apply(buf0, delta)) === 0)
```

## License

This project is licensed under [the Apache-2.0
license](http://www.apache.org/licenses/LICENSE-2.0).

Copyright (c) 2019 Dropbox, Inc.

Copyright (c) 2016 bacher09, Artyom Pavlov (RustCrypto/hashes/MD4).
