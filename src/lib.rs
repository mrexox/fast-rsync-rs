#![deny(clippy::all)]

use fast_rsync::{Signature, SignatureOptions};
use napi::bindgen_prelude::*;
use std::path::Path;

#[macro_use]
extern crate napi_derive;

const LIMIT: usize = 4294967296; // 4GBi
const DEFAULT_OPTIONS: Options = Options {
  block_size: 1024,
  crypto_hash_size: 8,
};

/// Signature options.
#[derive(Copy, Clone, Debug)]
#[napi(object, js_name = "SignatureOptions")]
pub struct Options {
  /// The granularity of the signature.
  /// Smaller block sizes yield larger, but more precise, signatures.
  pub block_size: u32,
  /// The number of bytes to use from the MD4 hash. Must be at most 16.
  /// The larger this is, the less likely that a delta will be mis-applied.
  pub crypto_hash_size: u32,
}

/// Calculate signature of a file.
#[napi]
pub fn file_signature(src: String) -> Result<Buffer> {
  let data = std::fs::read(src).map_err(|e| Error::from_reason(e.to_string()))?;

  signature(data.into(), DEFAULT_OPTIONS)
}

/// Calculates a signature of a Buffer.
#[napi]
pub fn signature(data: Buffer, options: Options) -> Result<Buffer> {
  let signature = Signature::calculate(
    &data,
    SignatureOptions {
      block_size: options.block_size,
      crypto_hash_size: options.crypto_hash_size,
    },
  );

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

  let sig = file_signature(a)?;
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
