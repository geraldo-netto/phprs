// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

//! PHP 8.x syntax target (PHP 8.5 grammar), with one quoted-string echo execution.
#![forbid(unsafe_code)]

pub mod contracts;
mod execution;
pub mod execution_contracts;
mod syntax;

pub use execution::EchoExecutor;
pub use syntax::PhpParser;
