#![deny(clippy::all)]

use napi::bindgen_prelude::*;

use std::path::Path;

use fast_rsync::{Signature, SignatureOptions};

const LIMIT: usize = 4294967296;
const SIGNATURE_OPTIONS: SignatureOptions = SignatureOptions {
  block_size: 1024,
  crypto_hash_size: 8,
};

#[macro_use]
extern crate napi_derive;

/// Calculate signature of a file.
#[napi]
pub fn signature(src: String) -> Result<Buffer> {
  let data = std::fs::read(src).map_err(|e| Error::from_reason(e.to_string()))?;

  let signature = Signature::calculate(&data, SIGNATURE_OPTIONS);

  Ok(signature.into_serialized().into())
}

/// Calculate the diff between two files.
#[napi]
pub fn diff_files(a: String, b: String) -> Result<Buffer> {
  if !Path::new(&a).exists() {
    return Err(Error::from_reason(format!("file {a} does not exist")));
  }
  if !Path::new(&b).exists() {
    return Err(Error::from_reason(format!("file {b} does not exist")));
  }

  let sig = signature(a)?;
  let data = std::fs::read(b).map_err(|e| Error::from_reason(e.to_string()))?;

  diff(sig.into(), data.into())
}

/// Calculate the diff between a file signature and the raw data.
#[napi]
pub fn diff(signature: Buffer, buf: Buffer) -> Result<Buffer> {
  let signature =
    Signature::deserialize(signature.into()).map_err(|e| Error::from_reason(e.to_string()))?;
  let signature = signature.index();

  let buf: Vec<u8> = buf.into();
  let mut out: Vec<u8> = Vec::new();
  let _ =
    fast_rsync::diff(&signature, &buf, &mut out).map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(out.into())
}

/// Apply the delta to a file.
#[napi]
pub fn patch_file(dest: String, delta: Buffer) -> Result<Buffer> {
  if !Path::new(&dest).exists() {
    return Err(Error::from_reason(format!("file {dest} does not exist")));
  }

  let data = std::fs::read(dest).map_err(|e| Error::from_reason(e.to_string()))?;

  apply(data.into(), delta)
}

/// Apply the delta to a Buffer.
#[napi]
pub fn apply(data: Buffer, delta: Buffer) -> Result<Buffer> {
  let data: Vec<u8> = data.into();
  let mut out: Vec<u8> = Vec::new();
  let mut d: Vec<u8> = delta.into();
  let _ = fast_rsync::apply_limited(&data, &mut d, &mut out, LIMIT)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(out.into())
}
