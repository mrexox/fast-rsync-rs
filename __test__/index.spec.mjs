import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs'
import path from 'node:path'
import os from 'node:os'
import test from 'ava'

import { diff, diffFiles, signature, patchFile, apply } from '../index.js'

const FILE = '__test__/binary.dat'
const FILE_EDITED = '__test__/binary.dat.edited'


function tmpWrite(data) {
  let filename = path.join(mkdtempSync(path.join(os.tmpdir(), 'test-')), 'test-file')
  writeFileSync(filename, data)
  return filename
}

test('calculates the correct diff', (t) => {
  let delta = diffFiles(FILE, FILE_EDITED)
  let resultBuf = patchFile(FILE, delta)

  t.is(Buffer.compare(resultBuf, readFileSync(FILE_EDITED)), 0)
})

test('using a temporary signature file', (t) => {
  let sig = signature(FILE)
  let sigFilename = tmpWrite(sig)
  sig = readFileSync(sigFilename)

  let delta = diff(sig, readFileSync(FILE_EDITED))

  const source = readFileSync(FILE)
  let resultBuf = apply(source, delta)

  t.is(Buffer.compare(resultBuf, readFileSync(FILE_EDITED)), 0)
})
