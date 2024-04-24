import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs'
import path from 'node:path'
import os from 'node:os'
import test from 'ava'

import { diff, diffFiles, fileSignature, signature, patchFile, apply } from '../index.js'

const FILE = '__test__/binary.dat'
const FILE_EDITED = '__test__/binary.dat.edited'

function tmpWrite(data) {
  let filename = path.join(mkdtempSync(path.join(os.tmpdir(), 'test-')), 'test-file')
  writeFileSync(filename, data)
  return filename
}

test('calculates the correct diff', (t) => {
  const delta = diffFiles(FILE, FILE_EDITED)
  const resultBuf = patchFile(FILE, delta)

  t.is(Buffer.compare(resultBuf, readFileSync(FILE_EDITED)), 0)
})

test('uses a temporary signature file', (t) => {
  let sig = fileSignature(FILE)
  const sigFilename = tmpWrite(sig)
  sig = readFileSync(sigFilename)

  const dest = readFileSync(FILE_EDITED)
  const delta = diff(sig, dest)

  const source = readFileSync(FILE)
  const resultBuf = apply(source, delta)

  t.is(Buffer.compare(resultBuf, dest), 0)
})

test('uses custom options', (t) => {
  const source = readFileSync(FILE)
  const dest = readFileSync(FILE_EDITED)
  const sig = signature(source, { blockSize: 512, cryptoHashSize: 8 })

  const delta = diff(sig, dest)

  t.is(Buffer.compare(apply(source, delta), dest), 0)
})
