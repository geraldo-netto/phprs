// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

//! Independent contracts for the selected hello world execution delivery.
use crate::contracts::{Diagnostic, ParseOptions};

#[derive(Clone, Copy, Debug)]
pub struct ExecutionOptions {
    pub parse: ParseOptions,
    pub output_bytes: usize,
}

/// Synchronous execution producing bounded output before any host writes.
///
/// The selected subset is exactly one echo of a quoted, non-interpolated string.
/// Unsupported syntax or escapes return an Unsupported diagnostic. All parsing
/// limits apply, and output may not exceed output_bytes. Failure exposes no
/// output. Implementations retain no source borrows or invocation resources;
/// successful owned output transfers to the caller independently of source and
/// executor lifetimes. Host I/O and its possible partial writes belong to adapters.
pub trait SourceExecutor {
    fn execute(&self, source: &[u8], options: ExecutionOptions) -> Result<Vec<u8>, Diagnostic>;
}
