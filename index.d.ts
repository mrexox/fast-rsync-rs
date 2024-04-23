/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** Calculate signature of a file. */
export function signature(src: string): Buffer
/** Calculate the diff between two files. */
export function diffFiles(a: string, b: string): Buffer
/** Calculate the diff between a file signature and the raw data. */
export function diff(signature: Buffer, buf: Buffer): Buffer
/** Apply the delta to a file. */
export function patchFile(dest: string, delta: Buffer): Buffer
/** Apply the delta to a Buffer. */
export function apply(data: Buffer, delta: Buffer): Buffer
