//! # Boki
//!
//! boki is a tool to help humans working with accounting data, a la [plain text accounting](https://plaintextaccounting.org/).
//!
//! ## Module structure
//!
//! 1. Only root modules can be public.
//! 2. Public modules must never form a dependency cycle.
//! 3. Public modules must be used directly either by the binary, or by two or more other public modules.

pub mod ast;
pub mod compile;
pub mod evaluate;
pub mod lexparse;
pub mod output;
pub mod utils;
